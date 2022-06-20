#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::ecs::event::Events;
use bevy::prelude::*;
use rgis_task::Task;
use std::mem;

mod geojson;
mod tasks;

// System
fn load_geojson_file_handler(
    mut load_event_reader: ResMut<Events<rgis_events::LoadGeoJsonFileEvent>>,
    thread_pool: Res<bevy::tasks::AsyncComputeTaskPool>,
    mut commands: bevy::ecs::system::Commands,
    mut network_fetch_task_outcome: ResMut<
        bevy::ecs::event::Events<rgis_task::TaskFinishedEvent<rgis_network::NetworkFetchTask>>,
    >,
) {
    for event in network_fetch_task_outcome.drain() {
        match event.outcome {
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

                tasks::LoadGeoJsonFileTask {
                    geojson_source: geojson::GeoJsonSource::Path(geojson_file_path),
                    source_crs: crs,
                    name,
                }
                .spawn(&thread_pool, &mut commands);
            }
            rgis_events::LoadGeoJsonFileEvent::FromNetwork { url, crs, name } => {
                rgis_network::NetworkFetchTask { url, crs, name }
                    .spawn(&thread_pool, &mut commands);
            }
            rgis_events::LoadGeoJsonFileEvent::FromBytes {
                file_name,
                bytes,
                crs,
            } => {
                tasks::LoadGeoJsonFileTask {
                    geojson_source: geojson::GeoJsonSource::Bytes(bytes),
                    source_crs: crs,
                    name: file_name,
                }
                .spawn(&thread_pool, &mut commands);
            }
        }
    }
}

fn handle_load_geojson_file_task_finished_events(
    mut create_layer_event_writer: EventWriter<rgis_events::CreateLayerEvent>,
    mut load_geojson_file_task_finished_events: ResMut<
        bevy::ecs::event::Events<rgis_task::TaskFinishedEvent<tasks::LoadGeoJsonFileTask>>,
    >,
) {
    for event in load_geojson_file_task_finished_events.drain() {
        match event.outcome {
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
fn load_layers_from_cli(
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

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(rgis_task::TaskPlugin::<tasks::LoadGeoJsonFileTask>::new())
            .add_system(load_geojson_file_handler)
            .add_system(handle_load_geojson_file_task_finished_events);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_startup_system(load_layers_from_cli);
    }
}
