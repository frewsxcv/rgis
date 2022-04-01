#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::prelude::Component;
use std::{pin, future};

#[derive(Default)]
pub struct TaskPlugin<T: Task> {
    t: std::marker::PhantomData<T>,
}

impl<T: Task> TaskPlugin<T> {
    pub fn new() -> Self {
        TaskPlugin {
            t: std::marker::PhantomData,
        }
    }
}

impl<T: Task> bevy::prelude::Plugin for TaskPlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(check_system::<T>)
            .add_event::<TaskFinishedEvent<T>>();
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub type PerformReturn<Output> =
    pin::Pin<Box<dyn future::Future<Output = Output> + Send + 'static>>;
#[cfg(target_arch = "wasm32")]
pub type PerformReturn<Output> = pin::Pin<Box<dyn future::Future<Output = Output> + 'static>>;

pub trait Task: Sized + Send + Sync + 'static {
    type Outcome: Send + Sync + 'static;

    fn name(&self) -> String;

    fn perform(self) -> PerformReturn<Self::Outcome>;

    fn spawn(
        self,
        pool: &bevy::tasks::AsyncComputeTaskPool,
        commands: &mut bevy::ecs::system::Commands,
    ) {
        let (sender, receiver) = async_channel::unbounded::<Self::Outcome>();

        let task_name = self.name();
        let in_progress_task = InProgressTask {
            task_name: task_name.clone(),
        };

        pool.spawn(async move {
            let task_name = task_name.clone();
            bevy::log::info!("Starting task '{}'", task_name);
            let outcome = self.perform().await;
            bevy::log::info!("Completed task '{}'", task_name);
            sender.send(outcome).await.unwrap();
        })
        .detach();

        commands
            .spawn()
            .insert(in_progress_task)
            .insert(InProgressTaskOutcomeReceiver::<Self>(receiver));
    }
}

pub fn check_system<T: Task>(
    query: bevy::ecs::system::Query<(&InProgressTaskOutcomeReceiver<T>, bevy::ecs::entity::Entity)>,
    mut commands: bevy::ecs::system::Commands,
    mut event_writer: bevy::ecs::event::EventWriter<TaskFinishedEvent<T>>,
) {
    for (receiver, entity) in query.iter() {
        if let Ok(outcome) = receiver.0.try_recv() {
            bevy::log::info!("Task finished");
            commands.entity(entity).despawn();
            event_writer.send(TaskFinishedEvent { outcome });
        }
    }
}

#[derive(Component)]
pub struct InProgressTask {
    pub task_name: String,
}

#[derive(Component)]
pub struct InProgressTaskOutcomeReceiver<T: Task>(async_channel::Receiver<T::Outcome>);

pub struct TaskFinishedEvent<T: Task> {
    pub outcome: T::Outcome,
}
