pub struct LoadFileJob {
    pub file_format: geo_file_loader::FileFormat,
    pub bytes: bytes::Bytes,
    pub name: String,
    pub source_crs: rgis_primitives::Crs,
}

pub enum LoadFileJobOutcome {
    Vector {
        loaded_file: geo_file_loader::LoadedFile,
        name: String,
        source_crs: rgis_primitives::Crs,
    },
    Raster {
        raster: geo_raster::Raster,
        name: String,
        source_crs: rgis_primitives::Crs,
    },
}

impl bevy_jobs::Job for LoadFileJob {
    type Outcome = Result<LoadFileJobOutcome, geo_file_loader::Error>;
    const JOB_TYPE: bevy_jobs::JobType = bevy_jobs::JobType::Io;

    fn name(&self) -> String {
        format!("Loading {} file", self.file_format.display_name())
    }

    async fn perform(self, _: bevy_jobs::Context) -> Self::Outcome {
        if self.file_format.is_raster() {
            let raster = geo_file_loader::load_raster_file(self.bytes).await?;
            Ok(LoadFileJobOutcome::Raster {
                raster,
                name: self.name,
                source_crs: self.source_crs,
            })
        } else {
            let loaded_file = geo_file_loader::load_file(self.file_format, self.bytes)?;
            Ok(LoadFileJobOutcome::Vector {
                loaded_file,
                name: self.name,
                source_crs: self.source_crs,
            })
        }
    }
}
