use bevy::ecs::event::Events;
use bevy::prelude::*;

fn handle_network_fetch_finished_tasks<F: geo_file_loader::FileLoader + Send + Sync + 'static>(
    mut load_event_reader: ResMut<Events<rgis_events::LoadFileEvent<F>>>,
    mut finished_tasks: bevy_jobs::FinishedJobs,
) where
    <F as geo_file_loader::FileLoader>::Error: Send + Sync + 'static,
{
    while let Some(outcome) = finished_tasks.take_next::<rgis_network::NetworkFetchTask>() {
        match outcome {
            Ok(fetched) => load_event_reader.send(rgis_events::LoadFileEvent::FromBytes {
                file_loader: F::from_bytes(fetched.bytes),
                file_name: fetched.name,
                crs: fetched.crs,
            }),
            Err(e) => {
                bevy::log::error!("Could not fetch file: {:?}", e);
            }
        }
    }
}

fn handle_load_file_events<F: geo_file_loader::FileLoader + Send + Sync + 'static>(
    mut load_event_reader: ResMut<Events<rgis_events::LoadFileEvent<F>>>,
    mut task_spawner: bevy_jobs::JobSpawner,
) where
    <F as geo_file_loader::FileLoader>::Error: Send + Sync + 'static,
{
    for event in load_event_reader.drain() {
        match event {
            rgis_events::LoadFileEvent::FromNetwork { url, crs, name } => {
                task_spawner.spawn(rgis_network::NetworkFetchTask { url, crs, name })
            }
            rgis_events::LoadFileEvent::FromBytes {
                file_name,
                file_loader,
                crs,
            } => task_spawner.spawn(crate::tasks::LoadFileJob {
                file_loader,
                source_crs: crs,
                name: file_name,
            }),
        }
    }
}

fn handle_load_file_task_finished_events<F: geo_file_loader::FileLoader + Send + Sync + 'static>(
    mut finished_tasks: bevy_jobs::FinishedJobs,
    mut create_layer_event_writer: EventWriter<rgis_events::CreateLayerEvent>,
) where
    <F as geo_file_loader::FileLoader>::Error: Send + Sync + 'static,
{
    while let Some(outcome) = finished_tasks.take_next::<crate::tasks::LoadFileJob<F>>() {
        match outcome {
            Ok(outcome) => create_layer_event_writer.send(rgis_events::CreateLayerEvent {
                name: outcome.name,
                feature_collection: outcome.feature_collection,
                source_crs: outcome.source_crs,
            }),
            Err(e) => {
                bevy::log::error!("Encountered error when loading file: {:?}", e);
            }
        }
    }
}

pub fn system_set() -> SystemSet {
    SystemSet::new()
        .with_system(handle_network_fetch_finished_tasks::<geo_file_loader::GeoJsonSource>)
        .with_system(handle_load_file_events::<geo_file_loader::GeoJsonSource>)
        .with_system(handle_load_file_events::<geo_file_loader::WktSource>)
        .with_system(handle_load_file_task_finished_events::<geo_file_loader::GeoJsonSource>)
        .with_system(handle_load_file_task_finished_events::<geo_file_loader::WktSource>)
}
