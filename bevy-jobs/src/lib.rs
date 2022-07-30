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
            .insert_resource(JobOutcomePayloads(vec![]));
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

    fn spawn(self, commands: &mut bevy::ecs::system::Commands) {
        let (sender, receiver) = async_channel::unbounded::<JobOutcomePayload>();

        let job_name = self.name();
        let in_progress_job = InProgressJob {
            name: job_name.clone(),
            recv: receiver,
        };

        bevy::tasks::AsyncComputeTaskPool::get()
            .spawn(async move {
                let instant = instant::Instant::now();
                bevy::log::info!("Starting job '{}'", job_name);
                let outcome = self.perform().await;
                bevy::log::info!("Completed job '{}' in {:?}", job_name, instant.elapsed());
                if let Err(e) = sender
                    .send(JobOutcomePayload {
                        job_outcome_type_id: any::TypeId::of::<Self>(),
                        job_outcome: Box::new(outcome),
                    })
                    .await
                {
                    bevy::log::error!(
                        "Failed to send result from job {} back to main thread: {:?}",
                        job_name,
                        e
                    );
                }
            })
            .detach();

        commands.spawn().insert(in_progress_job);
    }
}

fn check_system(
    query: bevy::ecs::system::Query<(&InProgressJob, bevy::ecs::entity::Entity)>,
    mut commands: bevy::ecs::system::Commands,
    mut finished_jobs: FinishedJobs,
) {
    query.for_each(|(receiver, entity)| {
        if let Ok(outcome) = receiver.recv.try_recv() {
            bevy::log::info!("Job finished");
            commands.entity(entity).despawn();
            finished_jobs.outcomes.0.push(outcome);
        }
    })
}

struct JobOutcomePayload {
    job_outcome_type_id: any::TypeId,
    job_outcome: Box<dyn any::Any + Send + Sync>,
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct JobSpawner<'w, 's> {
    commands: bevy::ecs::system::Commands<'w, 's>,
}

impl<'w, 's> JobSpawner<'w, 's> {
    pub fn spawn<J: Job>(&mut self, job: J) {
        job.spawn(&mut self.commands)
    }
}

#[derive(Component)]
pub struct InProgressJob {
    pub name: String,
    recv: async_channel::Receiver<JobOutcomePayload>,
}

#[derive(bevy::ecs::system::SystemParam)]
pub struct FinishedJobs<'w, 's> {
    outcomes: bevy::ecs::system::ResMut<'w, JobOutcomePayloads>,
    #[system_param(ignore)]
    phantom_data: std::marker::PhantomData<&'s ()>,
}

pub struct JobOutcomePayloads(Vec<JobOutcomePayload>);

impl<'w, 's> FinishedJobs<'w, 's> {
    #[inline]
    pub fn take_next<J: Job>(&mut self) -> Option<J::Outcome> {
        let index = self
            .outcomes
            .0
            .iter_mut()
            .enumerate()
            .filter(|(_i, outcome_payload)| {
                any::TypeId::of::<J>() == outcome_payload.job_outcome_type_id
                    && outcome_payload.job_outcome.is::<J::Outcome>()
            })
            .map(|(i, _)| i)
            .next()?;
        let outcome_payload = self.outcomes.0.remove(index);
        let outcome = outcome_payload.job_outcome.downcast::<J::Outcome>();
        if outcome.is_err() {
            bevy::log::error!("encountered unexpected job result type");
        }
        outcome.map(|n| *n).ok()
    }
}
