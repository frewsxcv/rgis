use bevy::prelude::*;

pub struct OpenFileJob;

impl bevy_jobs::Job for OpenFileJob {
    type Outcome = Option<OpenedFile>;
    const JOB_TYPE: bevy_jobs::JobType = bevy_jobs::JobType::Io;

    fn name(&self) -> String {
        "Opening file".into()
    }

    async fn perform(self, _: bevy_jobs::Context) -> Self::Outcome {
        let task = rfd::AsyncFileDialog::new().pick_file();
        let file_handle = task.await?;
        let file_name = file_handle.file_name();
        let bytes = file_handle.read().await;
        Some(OpenedFile { file_name, bytes })
    }
}

#[derive(Default, Resource)]
pub struct SelectedFile(pub Option<OpenedFile>);

pub struct OpenedFile {
    pub bytes: Vec<u8>,
    pub file_name: String,
}
