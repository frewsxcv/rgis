use bevy::app::Events;
use bevy::prelude::*;
use std::path;

mod geojson;

#[derive(Debug)]
pub struct LoadGeoJsonFile {
    pub path: path::PathBuf,
    pub source_srs: String,
    pub target_srs: String,
}

struct SpawnedLayers(Vec<rgis_layers::LayerId>);

type LoadedGeoJsonFileSender = async_channel::Sender<SpawnedLayers>;
type LoadedGeoJsonFileReceiver = async_channel::Receiver<SpawnedLayers>;

// System
fn load_geojson_file_handler(
    layers: rgis_layers::ResLayers,
    mut load_event_reader: EventReader<LoadGeoJsonFile>,
    thread_pool: Res<bevy::tasks::AsyncComputeTaskPool>,
    sender: Res<LoadedGeoJsonFileSender>,
) {
    for LoadGeoJsonFile {
        path: geojson_file_path,
        source_srs,
        target_srs,
    } in load_event_reader.iter()
    {
        let layers = layers.clone();
        let geojson_file_path = geojson_file_path.clone();
        let source_srs = source_srs.clone();
        let target_srs = target_srs.clone();
        let sender: LoadedGeoJsonFileSender = sender.clone();
        thread_pool
            .spawn(async move {
                let spawned_layers = SpawnedLayers(geojson::load(
                    geojson_file_path,
                    &mut layers.write().unwrap(),
                    &source_srs,
                    &target_srs,
                ));
                sender.send(spawned_layers).await.unwrap();
            })
            .detach();
    }
}

fn handle_loaded_layers(
    mut loaded_events: ResMut<Events<rgis_layers::LayerLoaded>>,
    receiver: Res<LoadedGeoJsonFileReceiver>,
) {
    while let Ok(layer_ids) = receiver.try_recv() {
        for layer_id in layer_ids.0 {
            loaded_events.send(rgis_layers::LayerLoaded(layer_id));
        }
    }
}

fn load_layers_from_cli(
    cli_values: Res<rgis_cli::Values>,
    mut events: ResMut<Events<LoadGeoJsonFile>>,
) {
    for geojson_file_path in &cli_values.geojson_files {
        debug!(
            "sending LoadGeoJsonFile event: {}",
            &geojson_file_path.display(),
        );
        events.send(LoadGeoJsonFile {
            path: geojson_file_path.clone(),
            source_srs: cli_values.source_srs.clone(),
            target_srs: cli_values.target_srs.clone(),
        });
    }
}

pub struct RgisFileLoaderPlugin;

impl Plugin for RgisFileLoaderPlugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver): (LoadedGeoJsonFileSender, LoadedGeoJsonFileReceiver) =
            async_channel::unbounded();
        app.add_event::<LoadGeoJsonFile>()
            .insert_resource(sender)
            .insert_resource(receiver)
            .add_startup_system(load_layers_from_cli.system())
            .add_system(load_geojson_file_handler.system())
            .add_system(handle_loaded_layers.system());
    }
}
