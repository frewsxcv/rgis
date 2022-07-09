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
pub type AsyncReturn<Output> = pin::Pin<Box<dyn future::Future<Output = Output> + Send + 'static>>;
#[cfg(target_arch = "wasm32")]
pub type AsyncReturn<Output> = pin::Pin<Box<dyn future::Future<Output = Output> + 'static>>;

pub trait Job: any::Any + Sized + Send + Sync + 'static {
    type Outcome: any::Any + Send + Sync;

    fn name(&self) -> String;

    fn perform(self) -> AsyncReturn<Self::Outcome>;

    fn spawn(
        self,
        pool: &bevy::tasks::AsyncComputeTaskPool,
        commands: &mut bevy::ecs::system::Commands,
    ) {
        let (sender, receiver) = async_channel::unbounded::<TaskOutcomePayload>();

        let task_name = self.name();
        let in_progress_task = InProgressJob {
            name: task_name.clone(),
            recv: receiver,
        };

        pool.spawn(async move {
            let instant = instant::Instant::now();
            bevy::log::info!("Starting task '{}'", task_name);
            let outcome = self.perform().await;
            bevy::log::info!("Completed task '{}' in {:?}", task_name, instant.elapsed());
            if let Err(e) = sender
                .send(TaskOutcomePayload {
                    task_outcome_type_id: any::TypeId::of::<Self>(),
                    task_outcome: Box::new(outcome),
                })
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
    query: bevy::ecs::system::Query<(&InProgressJob, bevy::ecs::entity::Entity)>,
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

struct TaskOutcomePayload {
    task_outcome_type_id: any::TypeId,
    task_outcome: Box<dyn any::Any + Send + Sync>,
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct JobSpawner<'w, 's> {
    thread_pool: bevy::ecs::system::Res<'w, bevy::tasks::AsyncComputeTaskPool>,
    commands: bevy::ecs::system::Commands<'w, 's>,
}

impl<'w, 's> JobSpawner<'w, 's> {
    pub fn spawn<T: Job>(&mut self, task: T) {
        task.spawn(&self.thread_pool, &mut self.commands)
    }
}

#[derive(Component)]
pub struct InProgressJob {
    pub name: String,
    recv: async_channel::Receiver<TaskOutcomePayload>,
}

pub struct FinishedJobs {
    outcomes: Vec<TaskOutcomePayload>,
}

impl FinishedJobs {
    #[inline]
    pub fn take_next<T: Job>(&mut self) -> Option<T::Outcome> {
        let index = self
            .outcomes
            .iter_mut()
            .enumerate()
            .filter(|(_i, outcome_payload)| {
                any::TypeId::of::<T>() == outcome_payload.task_outcome_type_id
                    && outcome_payload.task_outcome.is::<T::Outcome>()
            })
            .map(|(i, _)| i)
            .next()?;
        let outcome_payload = self.outcomes.remove(index);
        let outcome = outcome_payload.task_outcome.downcast::<T::Outcome>();
        if outcome.is_err() {
            bevy::log::error!("encountered unexpected task result type");
        }
        outcome.map(|n| *n).ok()
    }
}
