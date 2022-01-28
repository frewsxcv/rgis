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

// System
fn load_geojson_file_handler(
    layers: rgis_layers::ResLayers,
    mut load_event_reader: EventReader<LoadGeoJsonFile>,
    mut loaded_events: ResMut<Events<rgis_layers::LayerLoaded>>,
) {
    for LoadGeoJsonFile {
        path: geojson_file_path,
        source_srs,
        target_srs,
    } in load_event_reader.iter()
    {
        let layer_ids = geojson::load(
            geojson_file_path.clone(),
            &mut layers.write().unwrap(),
            source_srs,
            target_srs,
        );
        for layer_id in layer_ids {
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
        app.add_event::<LoadGeoJsonFile>()
            .add_startup_system(load_layers_from_cli.system())
            .add_system(load_geojson_file_handler.system());
    }
}
