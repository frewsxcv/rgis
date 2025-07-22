use bevy::prelude::*;

fn handle_network_fetch_finished_jobs(
    mut load_event_reader: ResMut<Events<rgis_events::LoadFileEvent>>,
    mut finished_jobs: bevy_jobs::FinishedJobs,
) {
    while let Some(outcome) = finished_jobs.take_next::<rgis_network::NetworkFetchJob>() {
        match outcome {
            Ok(fetched) => {
                load_event_reader.send(rgis_events::LoadFileEvent::FromBytes {
                    file_format: geo_file_loader::FileFormat::GeoJson,
                    bytes: fetched.bytes,
                    file_name: fetched.name,
                    crs_epsg_code: fetched.crs_epsg_code,
                });
            }
            Err(e) => {
                bevy::log::error!("Could not fetch file: {:?}", e);
            }
        }
    }
}

fn handle_load_file_events(
    mut load_event_reader: ResMut<Events<rgis_events::LoadFileEvent>>,
    mut job_spawner: bevy_jobs::JobSpawner,
) {
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
                bytes,
                file_format,
                crs_epsg_code,
            } => job_spawner.spawn(crate::jobs::LoadFileJob {
                source_crs_epsg_code: crs_epsg_code,
                name: file_name,
                bytes,
                file_format,
            }),
        };
    }
}

fn handle_load_file_job_finished_events(
    mut finished_jobs: bevy_jobs::FinishedJobs,
    mut create_layer_event_writer: EventWriter<rgis_events::CreateLayerEvent>,
) {
    while let Some(outcome) = finished_jobs.take_next::<crate::jobs::LoadFileJob>() {
        match outcome {
            Ok(outcome) => {
                create_layer_event_writer.write(rgis_events::CreateLayerEvent {
                    name: outcome.name,
                    feature_collection: outcome.feature_collection,
                    source_crs_epsg_code: outcome.source_crs_epsg_code,
                });
            }
            Err(e) => {
                bevy::log::error!("Encountered error when loading file: {:?}", e);
            }
        }
    }
}

fn handle_lod_builder_job_finished_events(
    mut finished_jobs: bevy_jobs::FinishedJobs,
    mut layers: ResMut<rgis_layers::Layers>,
    mut layer_reprojected_event_writer: EventWriter<rgis_events::LayerReprojectedEvent>,
) {
    while let Some(outcome) = finished_jobs.take_next::<crate::lod_builder::LODBuilderJob>() {
        match outcome {
            Ok(outcome) => {
                let Some(layer) = layers.get_mut(outcome.layer_id) else {
                    continue;
                };
                layer.projected_feature_collection = Some(outcome.lods);
                layer_reprojected_event_writer
                    .write(rgis_events::LayerReprojectedEvent(outcome.layer_id));
            }
            Err(e) => {
                bevy::log::error!("Encountered error when building LODs: {:?}", e);
            }
        }
    }
}

pub fn configure(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_network_fetch_finished_jobs,
            handle_load_file_events,
            handle_load_file_job_finished_events,
            handle_lod_builder_job_finished_events,
        ),
    );
}
