use geo_features::FeatureCollection;
use geo_projected::WrapTo;
use geo_raster::Raster;

pub struct LoadFileJob {
    pub file_format: geo_file_loader::FileFormat,
    pub bytes: bytes::Bytes,
    pub name: String,
    pub source_crs_epsg_code: u16,
}

pub enum LoadedData {
    Vector(FeatureCollection<geo_projected::UnprojectedScalar>),
    Raster(Raster),
}

pub struct LoadFileJobOutcome {
    pub data: LoadedData,
    pub name: String,
    pub source_crs_epsg_code: u16,
}

impl bevy_jobs::Job for LoadFileJob {
    type Outcome = Result<LoadFileJobOutcome, geo_file_loader::Error>;
    const JOB_TYPE: bevy_jobs::JobType = bevy_jobs::JobType::Io;

    fn name(&self) -> String {
        format!("Loading {} file", self.file_format.display_name())
    }

    async fn perform(self, _: bevy_jobs::Context) -> Self::Outcome {
        let data = match self.file_format {
            geo_file_loader::FileFormat::GeoJson
            | geo_file_loader::FileFormat::Shapefile
            | geo_file_loader::FileFormat::Wkt
            | geo_file_loader::FileFormat::Gpx => {
                let features = geo_file_loader::load_vector_file(self.file_format, self.bytes)?
                    .into_iter()
                    .map(geo_file_loader_feature_to_geo_features_feature)
                    .collect();
                LoadedData::Vector(FeatureCollection::from_features(features).wrap())
            }
            geo_file_loader::FileFormat::GeoTiff => {
                let raster = geo_file_loader::load_raster_file(self.file_format, self.bytes)?;
                LoadedData::Raster(raster)
            }
        };

        Ok(LoadFileJobOutcome {
            data,
            name: self.name,
            source_crs_epsg_code: self.source_crs_epsg_code,
        })
    }
}

fn geo_file_loader_feature_to_geo_features_feature(
    feature: geo_file_loader::Feature,
) -> geo_features::Feature<f64> {
    let geometry = feature.geometry;
    let properties = feature
        .properties
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                geo_file_loader_owned_column_value_to_geo_features_value(v),
            )
        })
        .collect();
    geo_features::FeatureBuilder::new()
        .with_geometry(geometry)
        .with_properties(properties)
        .build()
}

fn geo_file_loader_owned_column_value_to_geo_features_value(
    value: geo_file_loader::OwnedColumnValue,
) -> geo_features::Value {
    match value {
        geo_file_loader::OwnedColumnValue::String(s) => geo_features::Value::String(s),
        geo_file_loader::OwnedColumnValue::Int(n) => geo_features::Value::Number(n.into()),
        geo_file_loader::OwnedColumnValue::UInt(n) => geo_features::Value::Number(n.into()),
        geo_file_loader::OwnedColumnValue::Long(n) => geo_features::Value::Number(n as f64),
        geo_file_loader::OwnedColumnValue::ULong(n) => geo_features::Value::Number(n as f64),
        geo_file_loader::OwnedColumnValue::Float(n) => geo_features::Value::Number(n.into()),
        geo_file_loader::OwnedColumnValue::Double(n) => geo_features::Value::Number(n),
        geo_file_loader::OwnedColumnValue::Bool(b) => geo_features::Value::Boolean(b),
        // TODO: fill in the other implementations here
        _ => unimplemented!(),
    }
}
