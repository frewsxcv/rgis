#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::ecs::event::Events;
use bevy::prelude::*;
use rgis_task::Task;
use std::{io, mem};

mod geojson;

struct SpawnedLayers(Vec<rgis_layers::UnassignedLayer>);

struct FetchedFile {
    name: String,
    bytes: Vec<u8>,
    crs: String,
}

enum GeoJsonSource {
    #[cfg(not(target_arch = "wasm32"))]
    Path(std::path::PathBuf),
    Bytes {
        file_name: String,
        bytes: Vec<u8>,
    },
}

struct LoadGeoJsonFileTask {
    geojson_source: GeoJsonSource,
    source_crs: String,
    target_crs: String,
}

impl rgis_task::Task for LoadGeoJsonFileTask {
    type Outcome = Result<SpawnedLayers, geojson::LoadGeoJsonError>;

    fn name(&self) -> String {
        "Loading GeoJson file".into()
    }

    fn perform(self) -> rgis_task::PerformReturn<Self::Outcome> {
        Box::pin(async move {
            Ok(SpawnedLayers(match self.geojson_source {
                #[cfg(not(target_arch = "wasm32"))]
                GeoJsonSource::Path(path) => {
                    geojson::load_from_path(&path, &self.source_crs, &self.target_crs)?
                }
                GeoJsonSource::Bytes { file_name, bytes } => geojson::load_from_reader(
                    io::Cursor::new(bytes),
                    file_name,
                    &self.source_crs,
                    &self.target_crs,
                )?,
            }))
        })
    }
}

type FetchedFileSender = async_channel::Sender<Result<FetchedFile, String>>;
type FetchedFileReceiver = async_channel::Receiver<Result<FetchedFile, String>>;

// System
fn load_geojson_file_handler(
    mut load_event_reader: ResMut<Events<rgis_events::LoadGeoJsonFileEvent>>,
    thread_pool: Res<bevy::tasks::AsyncComputeTaskPool>,
    rgis_settings: Res<rgis_settings::RgisSettings>,
    fetched_bytes_sender: Res<FetchedFileSender>,
    fetched_bytes_receiver: Res<FetchedFileReceiver>,
    mut commands: bevy::ecs::system::Commands,
) {
    while let Ok(fetched) = fetched_bytes_receiver.try_recv() {
        match fetched {
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
                LoadGeoJsonFileTask {
                    geojson_source: GeoJsonSource::Path(geojson_file_path),
                    source_crs: crs,
                    target_crs: rgis_settings.target_crs.clone(),
                }
                .spawn(&thread_pool, &mut commands);
            }
            rgis_events::LoadGeoJsonFileEvent::FromNetwork { url, crs, name } => {
                let fetched_bytes_sender = fetched_bytes_sender.clone();
                let request = ehttp::Request::get(url);
                ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
                    if let Err(e) = fetched_bytes_sender.try_send(result.map(|r| FetchedFile {
                        bytes: r.bytes,
                        crs,
                        name,
                    })) {
                        bevy::log::error!(
                            "Failed to send network response to main thread: {:?}",
                            e
                        );
                    }
                });
            }
            rgis_events::LoadGeoJsonFileEvent::FromBytes {
                file_name,
                bytes,
                crs,
            } => {
                LoadGeoJsonFileTask {
                    geojson_source: GeoJsonSource::Bytes { bytes, file_name },
                    source_crs: crs,
                    target_crs: rgis_settings.target_crs.clone(),
                }
                .spawn(&thread_pool, &mut commands);
            }
        }
    }
}

fn handle_loaded_layers(
    mut loaded_events: EventWriter<rgis_events::LayerLoadedEvent>,
    mut layers: ResMut<rgis_layers::Layers>,
    mut task_finished: ResMut<
        bevy::ecs::event::Events<rgis_task::TaskFinishedEvent<LoadGeoJsonFileTask>>,
    >,
) {
    for event in task_finished.drain() {
        match event.outcome {
            Ok(unassigned_layers) => {
                for unassigned_layer in unassigned_layers.0 {
                    let layer_id = layers.add(unassigned_layer);
                    loaded_events.send(rgis_events::LayerLoadedEvent(layer_id));
                }
            }
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
        let (sender2, receiver2): (FetchedFileSender, FetchedFileReceiver) =
            async_channel::unbounded();
        app.insert_resource(sender2)
            .insert_resource(receiver2)
            .add_plugin(rgis_task::TaskPlugin::<LoadGeoJsonFileTask>::new())
            .add_system(load_geojson_file_handler)
            .add_system(handle_loaded_layers);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_startup_system(load_layers_from_cli);
    }
}
