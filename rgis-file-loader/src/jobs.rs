use geo_features::FeatureCollection;
use geo_projected::WrapTo;

pub struct LoadFileJob {
    pub file_format: geo_file_loader::FileFormat,
    pub bytes: bytes::Bytes,
    pub name: String,
    pub source_crs: rgis_primitives::Crs,
}

pub struct LoadFileJobOutcome {
    pub feature_collection: FeatureCollection<geo_projected::UnprojectedScalar>,
    pub name: String,
    pub source_crs: rgis_primitives::Crs,
}

impl bevy_jobs::Job for LoadFileJob {
    type Outcome = Result<LoadFileJobOutcome, geo_file_loader::Error>;
    const JOB_TYPE: bevy_jobs::JobType = bevy_jobs::JobType::Io;

    fn name(&self) -> String {
        format!("Loading {} file", self.file_format.display_name())
    }

    async fn perform(self, _: bevy_jobs::Context) -> Self::Outcome {
        let features = geo_file_loader::load_file(self.file_format, self.bytes)?
            .into_iter()
            .map(geo_file_loader_feature_to_geo_features_feature)
            .collect();
        Ok(LoadFileJobOutcome {
            feature_collection: FeatureCollection::from_features(features).wrap(),
            name: self.name,
            source_crs: self.source_crs,
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
        geo_file_loader::OwnedColumnValue::Binary(b) => {
            geo_features::Value::String(format!("{b:?}"))
        }
        geo_file_loader::OwnedColumnValue::Json(j) => geo_features::Value::String(format!("{j:?}")),
        geo_file_loader::OwnedColumnValue::Byte(n) => geo_features::Value::Number(f64::from(n)),
        geo_file_loader::OwnedColumnValue::UByte(n) => geo_features::Value::Number(f64::from(n)),
        geo_file_loader::OwnedColumnValue::Short(n) => geo_features::Value::Number(f64::from(n)),
        geo_file_loader::OwnedColumnValue::UShort(n) => geo_features::Value::Number(f64::from(n)),
        geo_file_loader::OwnedColumnValue::DateTime(n) => geo_features::Value::String(n),
    }
}
