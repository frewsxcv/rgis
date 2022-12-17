#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

pub struct FetchedFile {
    pub name: String,
    pub bytes: bytes::Bytes,
    pub crs: String,
}

pub struct NetworkFetchTask {
    pub url: String,
    pub crs: String,
    pub name: String,
}

impl bevy_jobs::Job for NetworkFetchTask {
    type Outcome = Result<FetchedFile, String>;

    fn name(&self) -> String {
        format!("Fetching '{}'", self.name)
    }

    fn perform(self) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            runtime.block_on(async {
                let response = reqwest::get(self.url).await.unwrap();
                let bytes = response.bytes().await.unwrap();
                Ok(FetchedFile {
                    bytes,
                    crs: self.crs,
                    name: self.name,
                })
            })
        })
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, _app: &mut bevy::app::App) {}
}
