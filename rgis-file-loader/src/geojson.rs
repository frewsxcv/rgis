#[cfg(not(target_arch = "wasm32"))]
use std::path;
use std::{error, io};

#[cfg(not(target_arch = "wasm32"))]
pub fn load_from_path(
    geojson_file_path: &path::Path,
    source_projection: &str,
    target_projection: &str,
) -> Result<Vec<rgis_layers::UnassignedLayer>, Box<dyn error::Error + Send + Sync>> {
    use std::fs;
    let tl = time_logger::start!("Opening file: {:?}", geojson_file_path);
    let reader = io::BufReader::new(fs::File::open(&geojson_file_path)?);
    tl.finish();

    let file_name = geojson_file_path
        .file_name()
        .ok_or("Could not retrieve file name of GeoJSON path")
        .map(|s| s.to_string_lossy().into_owned())?;

    load_from_reader(reader, file_name, source_projection, target_projection)
}

pub fn load_from_reader<R: io::Read + io::Seek>(
    mut reader: R,
    file_name: String,
    source_projection: &str,
    target_projection: &str,
) -> Result<Vec<rgis_layers::UnassignedLayer>, Box<dyn error::Error + Send + Sync>> {
    let mut iter = geojson::FeatureIterator::new(&mut reader).peekable();

    let mut geo_geometry_collection: geo::GeometryCollection<f64>;

    if iter.peek().is_some() {
        let tl = time_logger::start!("Parsing file and converting to geo-types: {:?}", file_name);
        geo_geometry_collection = geo::GeometryCollection::default();
        for feature_result in iter {
            let feature = feature_result?;
            if let Some(geometry) = feature.geometry {
                geo_geometry_collection.0.push(geometry.try_into()?);
            }
        }
        tl.finish();
    } else {
        reader.rewind()?;
        let tl = time_logger::start!("Parsing file: {:?}", file_name);
        let geojson: geojson::GeoJson = serde_json::from_reader(reader)?;
        tl.finish();

        let tl = time_logger::start!("Converting to geo-types: {:?}", file_name);
        geo_geometry_collection = geojson::quick_collection(&geojson)?;
        tl.finish();
    };

    let unassigned_layer = rgis_layers::UnassignedLayer::from_geometry(
        geo::Geometry::GeometryCollection(geo_geometry_collection),
        file_name,
        None,
        source_projection,
        target_projection,
    )?;

    Ok(vec![unassigned_layer])
}
