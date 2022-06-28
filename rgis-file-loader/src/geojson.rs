#[cfg(not(target_arch = "wasm32"))]
use std::path;
use std::{io, iter};

pub enum GeoJsonSource {
    #[cfg(not(target_arch = "wasm32"))]
    Path(std::path::PathBuf),
    Bytes(Vec<u8>),
}

impl crate::FileLoader for GeoJsonSource {
    type Error = LoadGeoJsonError;

    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error> {
        Ok(match self {
            #[cfg(not(target_arch = "wasm32"))]
            GeoJsonSource::Path(path) => load_from_path(&path)?,
            GeoJsonSource::Bytes(bytes) => load_from_reader(io::Cursor::new(bytes))?,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_from_path(
    geojson_file_path: &path::Path,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    use std::fs;
    let tl = time_logger::start!("Opening file: {:?}", geojson_file_path);
    let reader = io::BufReader::new(fs::File::open(&geojson_file_path)?);
    tl.finish();

    load_from_reader(reader)
}

#[derive(thiserror::Error, Debug)]
pub enum LoadGeoJsonError {
    #[error("{0}")]
    GeoJson(#[from] geojson::Error),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}")]
    BoundingRect(#[from] geo_features::BoundingRectError),
}

fn attempt_to_load_with_feature_iterator<R: io::Read>(
    iter: iter::Peekable<geojson::FeatureIterator<R>>,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    let tl = time_logger::start!("Parsing file and converting to geo-types");
    let mut features: Vec<geo_features::Feature> = vec![];
    for feature_result in iter {
        let feature = feature_result?;
        features.push(geojson_feature_to_geo_feature(feature)?);
    }
    tl.finish();
    Ok(geo_features::FeatureCollection::from_features(features))
}

fn load_from_reader<R: io::Read + io::Seek>(
    mut reader: R,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    let mut iter = geojson::FeatureIterator::new(&mut reader).peekable();

    if iter.peek().is_some() {
        if let Ok(feature_collection) = attempt_to_load_with_feature_iterator(iter) {
            return Ok(feature_collection);
        } else {
            reader.rewind()?;
        }
    }

    let tl = time_logger::start!("Parsing file");
    let geojson: geojson::GeoJson = serde_json::from_reader(reader)?;
    tl.finish();

    let tl = time_logger::start!("Converting to geo-types");
    let feature_collection = match geojson {
        geojson::GeoJson::Geometry(g) => geojson_geometry_to_geo_feature_collection(g)?,
        geojson::GeoJson::Feature(f) => geojson_feature_to_geo_feature_collection(f)?,
        geojson::GeoJson::FeatureCollection(fc) => {
            geojson_feature_collection_to_geo_feature_collection(fc)?
        }
    };
    tl.finish();

    Ok(feature_collection)
}

fn geojson_geometry_to_geo_feature_collection(
    geojson_geometry: geojson::Geometry,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    let geo_geometry: geo::Geometry = geojson_geometry.try_into()?;
    let feature = geo_features::Feature::from_geometry(geo_geometry, Default::default())?;
    Ok(geo_features::FeatureCollection::from_feature(feature))
}

fn geojson_feature_to_geo_feature(
    geojson_feature: geojson::Feature,
) -> Result<geo_features::Feature, LoadGeoJsonError> {
    let geo_geometry: geo::Geometry = geojson_feature.geometry.unwrap().try_into()?;
    let properties = geojson_feature
        .properties
        .unwrap_or_default()
        .into_iter()
        .map(|(k, v)| (k, serde_json_value_to_geo_features_value(v)))
        .collect();
    Ok(geo_features::Feature::from_geometry(
        geo_geometry,
        properties,
    )?)
}

fn serde_json_value_to_geo_features_value(v: serde_json::Value) -> geo_features::Value {
    match v {
        serde_json::Value::Bool(b) => geo_features::Value::Boolean(b),
        serde_json::Value::Number(n) => geo_features::Value::Number(n.as_f64().unwrap()),
        serde_json::Value::String(s) => geo_features::Value::String(s),
        serde_json::Value::Null => geo_features::Value::Null,
        n => geo_features::Value::String(n.to_string()),
    }
}

fn geojson_feature_to_geo_feature_collection(
    geojson_feature: geojson::Feature,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    Ok(geo_features::FeatureCollection::from_feature(
        geojson_feature_to_geo_feature(geojson_feature)?,
    ))
}

fn geojson_feature_collection_to_geo_feature_collection(
    geojson_feature_collection: geojson::FeatureCollection,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    let mut features: Vec<geo_features::Feature> = vec![];
    for geojson_feature in geojson_feature_collection.features {
        features.push(geojson_feature_to_geo_feature(geojson_feature)?);
    }
    Ok(geo_features::FeatureCollection::from_features(features))
}
