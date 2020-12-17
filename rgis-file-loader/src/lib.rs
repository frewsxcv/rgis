use bevy::prelude::*;
use rgis_layers::Layers;

mod geojson;

#[derive(Debug)]
pub struct LoadGeoJsonFile {
    pub path: String,
    pub source_srs: String,
    pub target_srs: String,
}

// System
pub fn load_geojson_file_handler(
    mut layers: ResMut<rgis_layers::Layers>,
    load_events: Res<Events<LoadGeoJsonFile>>,
    mut load_event_reader: Local<EventReader<LoadGeoJsonFile>>,
    mut loaded_events: ResMut<Events<rgis_layers::LayerLoaded>>,
) {
    for LoadGeoJsonFile {
        path: geojson_file_path,
        source_srs,
        target_srs,
    } in load_event_reader.iter(&load_events)
    {
        let layer_ids = geojson::load(geojson_file_path.clone(), &mut layers, source_srs, target_srs);
        for layer_id in layer_ids {
            loaded_events.send(rgis_layers::LayerLoaded(layer_id));
        }
    }
}
