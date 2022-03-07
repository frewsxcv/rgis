#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use std::{io, path};

#[cfg(target_arch = "wasm32")]
pub fn load_from_path(
    geojson_file_path: path::PathBuf,
    source_projection: &str,
    target_projection: &str,
) -> Vec<rgis_layers::UnassignedLayer> {
    load_from_reader(
        std::io::Cursor::new(include_bytes!("../../sample-data/us-states.json")),
        String::from("test data us-states"),
        source_projection,
        target_projection,
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_from_path(
    geojson_file_path: path::PathBuf,
    source_projection: &str,
    target_projection: &str,
) -> Vec<rgis_layers::UnassignedLayer> {
    let tl = time_logger::start(format!("Opening file: {:?}", geojson_file_path));
    let reader = io::BufReader::new(fs::File::open(&geojson_file_path).expect("TODO"));
    tl.finish();

    let file_name = geojson_file_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();

    load_from_reader(
        reader,
        file_name,
        source_projection,
        target_projection,
    )
}

pub fn load_from_reader<R: io::Read>(
    reader: R,
    file_name: String,
    source_projection: &str,
    target_projection: &str,
) -> Vec<rgis_layers::UnassignedLayer> {
    let tl = time_logger::start(format!("Parsing file: {:?}", file_name));
    let geojson: geojson::GeoJson = serde_json::from_reader(reader).unwrap();
    tl.finish();

    let tl = time_logger::start(format!("Converting to geo-types: {:?}", file_name));
    let geo_geometry_collection: geo::GeometryCollection<f64> =
        geojson::quick_collection(&geojson).unwrap();
    tl.finish();

    let unassigned_layer = rgis_layers::UnassignedLayer::from_geometry(
        geo::Geometry::GeometryCollection(geo_geometry_collection),
        file_name,
        None,
        source_projection,
        target_projection,
    );

    vec![unassigned_layer]
}
