pub struct SaveFileJob {
    pub data: Vec<u8>,
    pub default_name: String,
    pub format: rgis_primitives::ExportFormat,
}

impl bevy_jobs::Job for SaveFileJob {
    type Outcome = Result<(), String>;
    const JOB_TYPE: bevy_jobs::JobType = bevy_jobs::JobType::Io;

    fn name(&self) -> String {
        "Saving file".into()
    }

    async fn perform(self, _: bevy_jobs::Context) -> Self::Outcome {
        let file_handle = rfd::AsyncFileDialog::new()
            .set_file_name(&self.default_name)
            .add_filter(self.format.label(), &[self.format.extension()])
            .save_file()
            .await;

        match file_handle {
            Some(handle) => handle
                .write(&self.data)
                .await
                .map_err(|e| e.to_string()),
            None => Ok(()), // User cancelled
        }
    }
}
