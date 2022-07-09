#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::Component;
use std::{any, future, pin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(check_system)
            .insert_resource(FinishedJobs { outcomes: vec![] });
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub type PerformReturn<Output> =
    pin::Pin<Box<dyn future::Future<Output = Output> + Send + 'static>>;
#[cfg(target_arch = "wasm32")]
pub type PerformReturn<Output> = pin::Pin<Box<dyn future::Future<Output = Output> + 'static>>;

pub trait Task: any::Any + Sized + Send + Sync + 'static {
    type Outcome: any::Any + Send + Sync;

    fn name(&self) -> String;

    fn perform(self) -> PerformReturn<Self::Outcome>;

    fn spawn(
        self,
        pool: &bevy::tasks::AsyncComputeTaskPool,
        commands: &mut bevy::ecs::system::Commands,
    ) {
        let (sender, receiver) = async_channel::unbounded::<OutcomePayload>();

        let task_name = self.name();
        let in_progress_task = InProgressTask {
            name: task_name.clone(),
            recv: receiver,
        };

        pool.spawn(async move {
            let instant = instant::Instant::now();
            bevy::log::info!("Starting task '{}'", task_name);
            let outcome = self.perform().await;
            bevy::log::info!("Completed task '{}' in {:?}", task_name, instant.elapsed());
            if let Err(e) = sender
                .send((any::TypeId::of::<Self>(), Box::new(outcome)))
                .await
            {
                bevy::log::error!(
                    "Failed to send result from task {} back to main thread: {:?}",
                    task_name,
                    e
                );
            }
        })
        .detach();

        commands.spawn().insert(in_progress_task);
    }
}

fn check_system(
    query: bevy::ecs::system::Query<(&InProgressTask, bevy::ecs::entity::Entity)>,
    mut commands: bevy::ecs::system::Commands,
    mut finished_tasks: bevy::ecs::system::ResMut<FinishedJobs>,
) {
    query.for_each(|(receiver, entity)| {
        if let Ok(outcome) = receiver.recv.try_recv() {
            bevy::log::info!("Task finished");
            commands.entity(entity).despawn();
            finished_tasks.outcomes.push(outcome);
        }
    })
}

// (<task type ID>, <task outcome value>)
type OutcomePayload = (any::TypeId, Box<dyn any::Any + Send + Sync>);

#[derive(bevy::ecs::system::SystemParam)]
pub struct JobSpawner<'w, 's> {
    thread_pool: bevy::ecs::system::Res<'w, bevy::tasks::AsyncComputeTaskPool>,
    commands: bevy::ecs::system::Commands<'w, 's>,
}

impl<'w, 's> JobSpawner<'w, 's> {
    pub fn spawn<T: Task>(&mut self, task: T) {
        task.spawn(&self.thread_pool, &mut self.commands)
    }
}

#[derive(Component)]
pub struct InProgressTask {
    pub name: String,
    recv: async_channel::Receiver<OutcomePayload>,
}

pub struct FinishedJobs {
    outcomes: Vec<OutcomePayload>,
}

impl FinishedJobs {
    #[inline]
    pub fn take_next<T: Task>(&mut self) -> Option<T::Outcome> {
        let index = self
            .outcomes
            .iter_mut()
            .enumerate()
            .filter(|(_i, (type_id, outcome))| {
                any::TypeId::of::<T>() == *type_id && outcome.is::<T::Outcome>()
            })
            .map(|(i, _)| i)
            .next()?;
        let (_type_id, x) = self.outcomes.remove(index);
        let outcome = x.downcast::<T::Outcome>();
        if outcome.is_err() {
            bevy::log::error!("encountered unexpected task result type");
        }
        outcome.map(|n| *n).ok()
    }
}
