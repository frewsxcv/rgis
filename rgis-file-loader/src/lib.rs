#![warn(clippy::unwrap_used, clippy::expect_used)]

use bevy::app::Events;
use bevy::prelude::*;
use rgis_task::Task;
use std::{error, io};

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
    type Outcome = Result<SpawnedLayers, Box<dyn error::Error + Send + Sync>>;

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

type FetchedFileSender = async_channel::Sender<FetchedFile>;
type FetchedFileReceiver = async_channel::Receiver<FetchedFile>;

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
        load_event_reader.send(rgis_events::LoadGeoJsonFileEvent::FromBytes {
            bytes: fetched.bytes,
            file_name: fetched.name,
            crs: fetched.crs,
        })
    }
    for event in load_event_reader.drain() {
        match event {
            #[cfg(not(target_arch = "wasm32"))]
            rgis_events::LoadGeoJsonFileEvent::FromPath {
                path: geojson_file_path,
                crs,
            } => {
                LoadGeoJsonFileTask {
                    geojson_source: GeoJsonSource::Path(geojson_file_path.clone()),
                    source_crs: crs.clone(),
                    target_crs: rgis_settings.target_crs.clone(),
                }
                .spawn(&thread_pool, &mut commands);
            }
            rgis_events::LoadGeoJsonFileEvent::FromNetwork { url, crs, name } => {
                let fetched_bytes_sender = fetched_bytes_sender.clone();
                let request = ehttp::Request::get(url);
                ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
                    let bytes = result.unwrap().bytes;
                    fetched_bytes_sender
                        .try_send(FetchedFile { bytes, crs, name })
                        .unwrap();
                });
            }
            rgis_events::LoadGeoJsonFileEvent::FromBytes {
                file_name,
                bytes,
                crs,
            } => {
                LoadGeoJsonFileTask {
                    geojson_source: GeoJsonSource::Bytes { bytes, file_name },
                    source_crs: crs.clone(),
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
    cli_values: Res<rgis_cli::Values>,
    mut events: EventWriter<rgis_events::LoadGeoJsonFileEvent>,
) {
    for geojson_file_path in &cli_values.geojson_files {
        debug!(
            "sending LoadGeoJsonFile event: {}",
            &geojson_file_path.display(),
        );
        events.send(rgis_events::LoadGeoJsonFileEvent::FromPath {
            path: geojson_file_path.clone(),
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
            .add_system(load_geojson_file_handler.system())
            .add_system(handle_loaded_layers.system());

        #[cfg(not(target_arch = "wasm32"))]
        app.add_startup_system(load_layers_from_cli.system());
    }
}
