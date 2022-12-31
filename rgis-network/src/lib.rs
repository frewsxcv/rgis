#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use std::io;

pub struct FetchedFile {
    pub name: String,
    pub bytes: bytes::Bytes,
    pub crs: String,
}

pub struct NetworkFetchJob {
    pub url: String,
    pub crs: String,
    pub name: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
}

impl bevy_jobs::Job for NetworkFetchJob {
    type Outcome = Result<FetchedFile, Error>;

    fn name(&self) -> String {
        format!("Fetching '{}'", self.name)
    }

    fn perform(self, _: bevy_jobs::Context) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            let fetch = async {
                let response = reqwest::get(self.url).await?;
                let bytes = response.bytes().await?;
                Ok(FetchedFile {
                    bytes,
                    crs: self.crs,
                    name: self.name,
                })
            };
            #[cfg(not(target_arch = "wasm32"))]
            {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(fetch)
            }
            #[cfg(target_arch = "wasm32")]
            {
                fetch.await
            }
        })
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, _app: &mut bevy::app::App) {}
}
