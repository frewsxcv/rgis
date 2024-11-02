pub struct LoadFileJob {
    pub file_format: geo_file_loader::FileFormat,
    pub bytes: bytes::Bytes,
    pub name: String,
    pub source_crs_epsg_code: u16,
}

pub struct LoadFileJobOutcome {
    pub feature_collection: geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
    pub name: String,
    pub source_crs_epsg_code: u16,
}

impl bevy_jobs::Job for LoadFileJob {
    type Outcome = Result<LoadFileJobOutcome, geo_file_loader::Error>;
    const JOB_TYPE: bevy_jobs::JobType = bevy_jobs::JobType::Io;

    fn name(&self) -> String {
        format!("Loading {} file", self.file_format.display_name())
    }

    fn perform(self, _: bevy_jobs::Context) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            let loaded_file = geo_file_loader::load_file(self.file_format, self.bytes)?;
            Ok(LoadFileJobOutcome {
                feature_collection: loaded_file.into(),
                name: self.name,
                source_crs_epsg_code: self.source_crs_epsg_code,
            })
        })
    }
}
