use bevy::ecs::event::Events;
use bevy::prelude::*;

fn handle_network_fetch_finished_jobs<F: geo_file_loader::FileLoader + Send + Sync + 'static>(
    mut load_event_reader: ResMut<Events<rgis_events::LoadFileEvent<F>>>,
    mut finished_jobs: bevy_jobs::FinishedJobs,
) where
    <F as geo_file_loader::FileLoader>::Error: Send + Sync + 'static,
{
    while let Some(outcome) = finished_jobs.take_next::<rgis_network::NetworkFetchJob>() {
        match outcome {
            Ok(fetched) => load_event_reader.send(rgis_events::LoadFileEvent::FromBytes {
                file_loader: F::from_bytes(fetched.bytes),
                file_name: fetched.name,
                crs_epsg_code: fetched.crs_epsg_code,
            }),
            Err(e) => {
                bevy::log::error!("Could not fetch file: {:?}", e);
            }
        }
    }
}

fn handle_load_file_events<F: geo_file_loader::FileLoader + Send + Sync + 'static>(
    mut load_event_reader: ResMut<Events<rgis_events::LoadFileEvent<F>>>,
    mut job_spawner: bevy_jobs::JobSpawner,
) where
    <F as geo_file_loader::FileLoader>::Error: Send + Sync + 'static,
{
    for event in load_event_reader.drain() {
        match event {
            rgis_events::LoadFileEvent::FromNetwork {
                url,
                crs_epsg_code,
                name,
            } => job_spawner.spawn(rgis_network::NetworkFetchJob {
                url,
                crs_epsg_code,
                name,
            }),
            rgis_events::LoadFileEvent::FromBytes {
                file_name,
                file_loader,
                crs_epsg_code,
            } => job_spawner.spawn(crate::jobs::LoadFileJob {
                file_loader,
                source_crs_epsg_code: crs_epsg_code,
                name: file_name,
            }),
        }
    }
}

fn handle_load_file_job_finished_events<F: geo_file_loader::FileLoader + Send + Sync + 'static>(
    mut finished_jobs: bevy_jobs::FinishedJobs,
    mut create_layer_event_writer: EventWriter<rgis_events::CreateLayerEvent>,
) where
    <F as geo_file_loader::FileLoader>::Error: Send + Sync + 'static,
{
    while let Some(outcome) = finished_jobs.take_next::<crate::jobs::LoadFileJob<F>>() {
        match outcome {
            Ok(outcome) => create_layer_event_writer.send(rgis_events::CreateLayerEvent {
                name: outcome.name,
                feature_collection: outcome.feature_collection,
                source_crs_epsg_code: outcome.source_crs_epsg_code,
            }),
            Err(e) => {
                bevy::log::error!("Encountered error when loading file: {:?}", e);
            }
        }
    }
}

pub fn configure(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_network_fetch_finished_jobs::<geo_file_loader::GeoJsonSource>,
            handle_load_file_events::<geo_file_loader::GeoJsonSource>,
            handle_load_file_events::<geo_file_loader::WktSource>,
            handle_load_file_events::<geo_file_loader::ShapefileSource>,
            handle_load_file_events::<geo_file_loader::GpxSource>,
            handle_load_file_job_finished_events::<geo_file_loader::GeoJsonSource>,
            handle_load_file_job_finished_events::<geo_file_loader::WktSource>,
            handle_load_file_job_finished_events::<geo_file_loader::ShapefileSource>,
            handle_load_file_job_finished_events::<geo_file_loader::GpxSource>,
        ),
    );
}
