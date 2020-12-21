use bevy::prelude::*;

mod geojson;

#[derive(Debug)]
pub struct LoadGeoJsonFile {
    pub path: String,
    pub source_srs: String,
    pub target_srs: String,
}

// System
fn load_geojson_file_handler(
    layers: rgis_layers::ResLayers,
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

pub struct RgisFileLoaderPlugin;

impl Plugin for RgisFileLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_event::<LoadGeoJsonFile>()
        .add_system(load_geojson_file_handler.system());
    }
}
