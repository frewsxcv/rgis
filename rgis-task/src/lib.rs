use bevy::prelude::Component;

pub trait Task: Sized + Send + 'static {
    type Outcome: Send + Sync;

    fn name(&self) -> String;
    fn perform(self) -> Self::Outcome;

    fn spawn(
        self,
        pool: &bevy::tasks::AsyncComputeTaskPool,
        commands: &mut bevy::ecs::system::Commands,
    ) {
        let (sender, receiver) = async_channel::unbounded::<Self::Outcome>();

        pool.spawn(async move {
            let task_name = self.name();
            bevy::log::info!("Starting task '{}'", task_name);
            let outcome = self.perform();
            bevy::log::info!("Completed task '{}'", task_name);
            sender.send(outcome).await.unwrap();
        })
        .detach();

        let inprogress: InProgressTask<Self> = InProgressTask { receiver };

        commands.spawn().insert(inprogress);
    }
}

pub fn check_system<T: Task>(
    query: bevy::ecs::system::Query<(&InProgressTask<T>, bevy::ecs::entity::Entity)>,
    mut commands: bevy::ecs::system::Commands,
    mut event_writer: bevy::ecs::event::EventWriter<TaskFinishedEvent<T>>,
) {
    for (in_progress_task, entity) in query.iter() {
        if let Ok(outcome) = in_progress_task.receiver.try_recv() {
            bevy::log::info!("Task finished");
            commands.entity(entity).despawn();
            event_writer.send(TaskFinishedEvent { outcome });
        }
    }
}

#[derive(Component)]
pub struct InProgressTask<T: Task> {
    pub receiver: async_channel::Receiver<T::Outcome>,
}

pub struct TaskFinishedEvent<T: Task> {
    pub outcome: T::Outcome,
}
