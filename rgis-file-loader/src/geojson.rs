use rgis_layers::Layers;
use std::{fs, io, path};

pub fn load(
    geojson_file_path: path::PathBuf,
    layers: &mut Layers,
    source_projection: &str,
    target_projection: &str,
) -> Vec<rgis_layers::LayerId> {
    let tl = time_logger::start(format!("Opening file: {:?}", geojson_file_path));
    let geojson_file = io::BufReader::new(fs::File::open(&geojson_file_path).expect("TODO"));
    tl.finish();

    let tl = time_logger::start(format!("Parsing file: {:?}", geojson_file_path));
    let geojson: geojson::GeoJson = serde_json::from_reader(geojson_file).unwrap();
    tl.finish();

    let tl = time_logger::start(format!("Converting to geo-types: {:?}", geojson_file_path));
    let geo_geometry_collection: geo_types::GeometryCollection<f64> =
        geojson::quick_collection(&geojson).unwrap();
    tl.finish();

    let tl = time_logger::start(format!("Adding new layer: {:?}", geojson_file_path));
    let layer_id = layers.add(
        geo_types::Geometry::GeometryCollection(geo_geometry_collection),
        geojson_file_path.file_name().unwrap().to_string_lossy().into_owned(),
        None,
        source_projection,
        target_projection,
    );
    tl.finish();

    vec![layer_id]
}
