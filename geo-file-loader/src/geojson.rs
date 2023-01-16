// create geo-file-loader

use std::{error, fmt, io, iter};

pub struct GeoJsonSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for GeoJsonSource {
    type Error = LoadGeoJsonError;

    const FILE_TYPE_NAME: &'static str = "GeoJSON";

    fn from_bytes(bytes: bytes::Bytes) -> Self {
        GeoJsonSource { bytes }
    }

    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error> {
        load_from_reader(io::Cursor::new(self.bytes))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LoadGeoJsonError {
    // https://github.com/georust/geojson/issues/197
    #[error("{0}")]
    GeoJson(#[from] Box<geojson::Error>),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("{0}")]
    BoundingRect(#[from] geo_features::BoundingRectError),
    #[error("{0}")]
    JsonNumberToFloat(#[from] JsonNumberToFloatError),
}

fn attempt_to_load_with_feature_iterator(
    iter: iter::Peekable<impl Iterator<Item = geojson::Result<geojson::Feature>>>,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    let tl = time_logger::start!("Parsing file and converting to geo-types");
    let mut features: Vec<geo_features::Feature> = vec![];
    for feature_result in iter {
        let feature = feature_result.map_err(Box::new)?;
        features.push(geojson_feature_to_geo_feature(feature)?);
    }
    tl.finish();
    Ok(geo_features::FeatureCollection::from_features(features))
}

fn load_from_reader<R: io::Read + io::Seek>(
    mut reader: R,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    {
        let mut iter = geojson::FeatureReader::from_reader(&mut reader)
            .features()
            .peekable();

        if iter.peek().is_some() {
            if let Ok(feature_collection) = attempt_to_load_with_feature_iterator(iter) {
                return Ok(feature_collection);
            } else {
                reader.rewind()?;
            }
        }
    }

    let tl = time_logger::start!("Parsing file");
    let geojson: geojson::GeoJson = serde_json::from_reader(reader)?;
    tl.finish();

    let tl = time_logger::start!("Converting to geo-types");
    let feature_collection = geojson_to_geo_feature_collection(geojson)?;
    tl.finish();

    Ok(feature_collection)
}

fn geojson_to_geo_feature_collection(
    geojson: geojson::GeoJson,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    Ok(match geojson {
        geojson::GeoJson::Geometry(g) => geojson_geometry_to_geo_feature_collection(g)?,
        geojson::GeoJson::Feature(f) => geojson_feature_to_geo_feature_collection(f)?,
        geojson::GeoJson::FeatureCollection(fc) => {
            geojson_feature_collection_to_geo_feature_collection(fc)?
        }
    })
}

fn geojson_geometry_to_geo_feature_collection(
    geojson_geometry: geojson::Geometry,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    let geo_geometry: geo::Geometry = geojson_geometry.try_into().map_err(Box::new)?;
    let feature = geo_features::FeatureBuilder::new()
        .with_geometry(geo_geometry)
        .build()?;
    Ok(geo_features::FeatureCollection::from_feature(feature))
}

fn geojson_feature_to_geo_feature(
    mut geojson_feature: geojson::Feature,
) -> Result<geo_features::Feature, LoadGeoJsonError> {
    let properties = geojson_feature_properties_to_geo_features_properties(&mut geojson_feature)?;
    let mut feature_builder = geo_features::FeatureBuilder::new().with_properties(properties);
    if let Some(geo_geometry) = geojson_feature
        .geometry
        .map(|geometry| geometry.try_into().map_err(Box::new))
        .transpose()?
    {
        feature_builder = feature_builder.with_geometry(geo_geometry);
    }
    Ok(feature_builder.build()?)
}

fn geojson_feature_properties_to_geo_features_properties(
    geojson_feature: &mut geojson::Feature,
) -> Result<geo_features::Properties, JsonNumberToFloatError> {
    let Some(properties) = geojson_feature.properties.take() else {
        return Ok(Default::default())
    };
    properties
        .into_iter()
        .map(|(k, v)| serde_json_value_to_geo_features_value(v).map(|v| (k, v)))
        .collect::<Result<geo_features::Properties, JsonNumberToFloatError>>()
}

#[derive(Debug)]
pub struct JsonNumberToFloatError;

impl fmt::Display for JsonNumberToFloatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not convert JSON number to f64")
    }
}

impl error::Error for JsonNumberToFloatError {}

fn serde_json_value_to_geo_features_value(
    v: serde_json::Value,
) -> Result<geo_features::Value, JsonNumberToFloatError> {
    Ok(match v {
        serde_json::Value::Bool(b) => geo_features::Value::Boolean(b),
        serde_json::Value::Number(n) => {
            geo_features::Value::Number(n.as_f64().ok_or(JsonNumberToFloatError)?)
        }
        serde_json::Value::String(s) => geo_features::Value::String(s),
        serde_json::Value::Null => geo_features::Value::Null,
        n => geo_features::Value::String(n.to_string()),
    })
}

fn geojson_feature_to_geo_feature_collection(
    geojson_feature: geojson::Feature,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    geojson_feature_to_geo_feature(geojson_feature)
        .map(geo_features::FeatureCollection::from_feature)
}

fn geojson_feature_collection_to_geo_feature_collection(
    geojson_feature_collection: geojson::FeatureCollection,
) -> Result<geo_features::FeatureCollection, LoadGeoJsonError> {
    geojson_feature_collection
        .features
        .into_iter()
        .map(geojson_feature_to_geo_feature)
        .collect::<Result<Vec<geo_features::Feature>, LoadGeoJsonError>>()
        .map(geo_features::FeatureCollection::from_features)
}
