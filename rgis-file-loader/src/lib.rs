use rgis_layers::Layers;

mod geojson;

pub fn load(
    file_path: String,
    layers: &mut Layers,
    source_projection: &str,
    target_projection: &str,
) -> Vec<rgis_layers::LayerId> {
    geojson::load(file_path, layers, source_projection, target_projection)
}
