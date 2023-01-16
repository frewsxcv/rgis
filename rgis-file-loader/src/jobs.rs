pub struct LoadFileJob<F: geo_file_loader::FileLoader> {
    pub file_loader: F,
    pub name: String,
    pub source_crs: String,
}

pub struct LoadFileJobOutcome {
    pub feature_collection: geo_projected::Unprojected<geo_features::FeatureCollection>,
    pub name: String,
    pub source_crs: String,
}

impl<F: geo_file_loader::FileLoader + Sync + Send + 'static> bevy_jobs::Job for LoadFileJob<F>
where
    <F as geo_file_loader::FileLoader>::Error: Send + Sync + 'static,
{
    type Outcome = Result<LoadFileJobOutcome, F::Error>;

    fn name(&self) -> String {
        format!("Loading {} file", F::FILE_TYPE_NAME)
    }

    fn perform(self, _: bevy_jobs::Context) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            Ok(LoadFileJobOutcome {
                feature_collection: geo_projected::Unprojected::new(self.file_loader.load()?),
                name: self.name,
                source_crs: self.source_crs,
            })
        })
    }
}
