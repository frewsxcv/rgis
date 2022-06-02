#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::prelude::Component;
use std::{future, pin};

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
pub type PerformReturn<Output> = pin::Pin<Box<dyn future::Future<Output = Output> + Send>>;
#[cfg(target_arch = "wasm32")]
pub type PerformReturn<Output> = pin::Pin<Box<dyn future::Future<Output = Output>>>;

pub trait Task: Sized + Send + Sync + 'static {
    type Outcome: Send + Sync;
    type Arguments: Send + Sync;

    fn name(&self) -> String;

    fn perform(args: Self::Arguments) -> PerformReturn<Self::Outcome>;

    fn spawn(
        self,
        args: Self::Arguments,
        pool: &bevy::tasks::AsyncComputeTaskPool,
        commands: &mut bevy::ecs::system::Commands,
    ) {
        let (sender, receiver) = async_channel::unbounded::<(Self::Outcome, Self)>();

        let task_name = self.name();

        pool.spawn(async move {
            bevy::log::info!("Starting task '{}'", task_name);
            let outcome = <Self as Task>::perform(args).await;
            bevy::log::info!("Completed task '{}'", task_name);
            if let Err(e) = sender.send((outcome, self)).await {
                bevy::log::error!(
                    "Failed to send result from task {} back to main thread: {:?}",
                    task_name,
                    e
                );
            }
        })
        .detach();

        commands
            .spawn()
            .insert(InProgressTaskOutcomeReceiver::<Self>(receiver));
    }
}

fn check_system<T: Task>(
    query: bevy::ecs::system::Query<(&InProgressTaskOutcomeReceiver<T>, bevy::ecs::entity::Entity)>,
    mut commands: bevy::ecs::system::Commands,
    mut event_writer: bevy::ecs::event::EventWriter<TaskFinishedEvent<T>>,
) {
    for (receiver, entity) in query.iter() {
        if let Ok((outcome, task)) = receiver.0.try_recv() {
            bevy::log::info!("Task finished");
            commands.entity(entity).despawn();
            event_writer.send(TaskFinishedEvent { outcome, task });
        }
    }
}

#[derive(Component)]
struct InProgressTaskOutcomeReceiver<T: Task>(async_channel::Receiver<(T::Outcome, T)>);

pub struct TaskFinishedEvent<T: Task> {
    pub outcome: T::Outcome,
    pub task: T,
}
