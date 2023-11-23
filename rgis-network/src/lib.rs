#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use futures_util::StreamExt;
use std::io;

#[cfg(not(target_arch = "wasm32"))]
lazy_static::lazy_static! {
    pub static ref TOKIO_RUNTIME: tokio::runtime::Runtime = {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    };
}

pub struct FetchedFile {
    pub name: String,
    pub bytes: bytes::Bytes,
    pub crs_epsg_code: u16,
}

pub struct NetworkFetchJob {
    pub url: String,
    pub crs_epsg_code: u16,
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

    fn perform(self, ctx: bevy_jobs::Context) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            await_future(async move {
                build_request_future(self.url, self.crs_epsg_code, self.name, ctx).await
            })
            .await
        })
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, _app: &mut bevy::app::App) {}
}

async fn build_request_future(
    url: String,
    crs_epsg_code: u16,
    name: String,
    ctx: bevy_jobs::Context,
) -> Result<FetchedFile, Error> {
    let response = reqwest::get(url).await?;
    let total_size = response.content_length().unwrap_or(0);
    let mut bytes_stream = response.bytes_stream();
    let mut bytes = Vec::<u8>::with_capacity(total_size as usize);
    let mut last_percent: u8 = 0;

    while let Some(bytes_chunk) = bytes_stream.next().await {
        let mut bytes_chunk = Vec::from(bytes_chunk?);
        bytes.append(&mut bytes_chunk);
        if total_size > 0 {
            let new_percent = (100 * bytes.len() / total_size as usize) as u8;
            if new_percent != last_percent {
                let _ = ctx.send_progress(new_percent).await;
                last_percent = new_percent;
            }
        }
    }

    Ok(FetchedFile {
        bytes: bytes::Bytes::from(bytes),
        crs_epsg_code,
        name,
    })
}

async fn await_future<Output>(
    future: impl std::future::Future<Output = Output> + 'static,
) -> Output {
    #[cfg(not(target_arch = "wasm32"))]
    {
        TOKIO_RUNTIME.block_on(future)
    }
    #[cfg(target_arch = "wasm32")]
    {
        fetch.await
    }
}
