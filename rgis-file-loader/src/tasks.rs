pub struct LoadGeoJsonFileTask {
    pub geojson_source: crate::geojson::GeoJsonSource,
    pub name: String,
    pub source_crs: String,
}

pub struct LoadGeoJsonFileTaskOutcome {
    pub geometry: geo::Geometry<f64>,
    pub name: String,
    pub source_crs: String,
}

impl rgis_task::Task for LoadGeoJsonFileTask {
    type Outcome = Result<LoadGeoJsonFileTaskOutcome, crate::geojson::LoadGeoJsonError>;

    fn name(&self) -> String {
        "Loading GeoJson file".into()
    }

    fn perform(self) -> rgis_task::PerformReturn<Self::Outcome> {
        Box::pin(async move {
            Ok(LoadGeoJsonFileTaskOutcome {
                geometry: self.geojson_source.load()?,
                name: self.name,
                source_crs: self.source_crs,
            })
        })
    }
}
