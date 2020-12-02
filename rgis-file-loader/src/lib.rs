use rgis_layers::Layers;

mod geojson;

pub fn load(
    file_path: String,
    layers: &mut Layers,
    source_projection: &'static str,
    target_projection: &'static str,
) -> usize {
    geojson::load(file_path, layers, source_projection, target_projection)
}
