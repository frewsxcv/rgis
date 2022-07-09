use bevy::ecs::event::Events;
use bevy::prelude::*;
use std::mem;

fn load_geojson_file_handler(
    mut load_event_reader: ResMut<Events<rgis_events::LoadGeoJsonFileEvent>>,
    mut task_spawner: bevy_jobs::JobSpawner,
    mut finished_tasks: bevy_jobs::FinishedJobs,
) {
    while let Some(outcome) = finished_tasks.take_next::<rgis_network::NetworkFetchTask>() {
        match outcome {
            Ok(fetched) => load_event_reader.send(rgis_events::LoadGeoJsonFileEvent::FromBytes {
                bytes: fetched.bytes,
                file_name: fetched.name,
                crs: fetched.crs,
            }),
            Err(e) => {
                bevy::log::error!("Could not fetch file: {:?}", e);
            }
        }
    }

    for event in load_event_reader.drain() {
        match event {
            #[cfg(not(target_arch = "wasm32"))]
            rgis_events::LoadGeoJsonFileEvent::FromPath {
                path: geojson_file_path,
                crs,
            } => {
                let name = geojson_file_path
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "<unknown>".to_string());

                task_spawner.spawn(crate::tasks::LoadGeoJsonFileTask {
                    geojson_source: crate::geojson::GeoJsonSource::Path(geojson_file_path),
                    source_crs: crs,
                    name,
                })
            }
            rgis_events::LoadGeoJsonFileEvent::FromNetwork { url, crs, name } => {
                task_spawner.spawn(rgis_network::NetworkFetchTask { url, crs, name })
            }
            rgis_events::LoadGeoJsonFileEvent::FromBytes {
                file_name,
                bytes,
                crs,
            } => task_spawner.spawn(crate::tasks::LoadGeoJsonFileTask {
                geojson_source: crate::geojson::GeoJsonSource::Bytes(bytes),
                source_crs: crs,
                name: file_name,
            }),
        }
    }
}

fn handle_load_geojson_file_task_finished_events(
    mut finished_tasks: bevy_jobs::FinishedJobs,
    mut create_layer_event_writer: EventWriter<rgis_events::CreateLayerEvent>,
) {
    while let Some(outcome) = finished_tasks.take_next::<crate::tasks::LoadGeoJsonFileTask>() {
        match outcome {
            Ok(outcome) => create_layer_event_writer.send(rgis_events::CreateLayerEvent {
                name: outcome.name,
                unprojected_geometry: outcome.geometry,
                source_crs: outcome.source_crs,
            }),
            Err(e) => {
                bevy::log::error!("Encountered error when loading GeoJSON file: {:?}", e);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_layers_from_cli(
    mut cli_values: ResMut<rgis_cli::Values>,
    mut events: EventWriter<rgis_events::LoadGeoJsonFileEvent>,
) {
    if let Some(geojson_stdin_bytes) = cli_values.geojson_stdin_bytes.take() {
        events.send(rgis_events::LoadGeoJsonFileEvent::FromBytes {
            bytes: geojson_stdin_bytes,
            crs: cli_values.source_crs.clone(),
            file_name: "Standard input".into(),
        })
    }
    for geojson_file_path in mem::take(&mut cli_values.geojson_files) {
        debug!(
            "sending LoadGeoJsonFile event: {}",
            &geojson_file_path.display(),
        );
        events.send(rgis_events::LoadGeoJsonFileEvent::FromPath {
            path: geojson_file_path,
            crs: cli_values.source_crs.clone(),
        });
    }
}

pub fn system_set() -> SystemSet {
    SystemSet::new()
        .with_system(load_geojson_file_handler)
        .with_system(handle_load_geojson_file_task_finished_events)
}
