use rgis_layers::Layers;
use std::{fs, io, path};

#[cfg(target_arch = "wasm32")]
pub fn load_from_path(
    geojson_file_path: path::PathBuf,
    layers: &mut Layers,
    source_projection: &str,
    target_projection: &str,
) -> Vec<rgis_layers::LayerId> {
    load_from_reader(
        std::io::Cursor::new(include_bytes!("../../sample-data/us-states.json")),
        String::from("test data us-states"),
        layers,
        source_projection,
        target_projection,
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_from_path(
    geojson_file_path: path::PathBuf,
    layers: &mut Layers,
    source_projection: &str,
    target_projection: &str,
) -> Vec<rgis_layers::LayerId> {
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
        layers,
        source_projection,
        target_projection,
    )
}

pub fn load_from_reader<R: io::Read>(
    reader: R,
    file_name: String,
    layers: &mut Layers,
    source_projection: &str,
    target_projection: &str,
) -> Vec<rgis_layers::LayerId> {
    let tl = time_logger::start(format!("Parsing file: {:?}", file_name));
    let geojson: geojson::GeoJson = serde_json::from_reader(reader).unwrap();
    tl.finish();

    let tl = time_logger::start(format!("Converting to geo-types: {:?}", file_name));
    let geo_geometry_collection: geo_types::GeometryCollection<f64> =
        geojson::quick_collection(&geojson).unwrap();
    tl.finish();

    let tl = time_logger::start(format!("Adding new layer: {:?}", file_name));
    let layer_id = layers.add(
        geo_types::Geometry::GeometryCollection(geo_geometry_collection),
        file_name,
        None,
        source_projection,
        target_projection,
    );
    tl.finish();

    vec![layer_id]
}
