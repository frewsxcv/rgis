use crate::FileLoader;

pub struct LoadGeoJsonFileTask {
    pub geojson_source: crate::geojson::GeoJsonSource,
    pub name: String,
    pub source_crs: String,
}

pub struct LoadGeoJsonFileTaskOutcome {
    pub geometry: geo_features::FeatureCollection,
    pub name: String,
    pub source_crs: String,
}

impl bevy_jobs::Task for LoadGeoJsonFileTask {
    type Outcome = Result<LoadGeoJsonFileTaskOutcome, crate::geojson::LoadGeoJsonError>;

    fn name(&self) -> String {
        "Loading GeoJson file".into()
    }

    fn perform(self) -> bevy_jobs::PerformReturn<Self::Outcome> {
        Box::pin(async move {
            Ok(LoadGeoJsonFileTaskOutcome {
                geometry: self.geojson_source.load()?,
                name: self.name,
                source_crs: self.source_crs,
            })
        })
    }
}
