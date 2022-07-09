#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

pub struct FetchedFile {
    pub name: String,
    pub bytes: Vec<u8>,
    pub crs: String,
}

type FetchedFileSender = async_channel::Sender<Result<ehttp::Response, String>>;
type FetchedFileReceiver = async_channel::Receiver<Result<ehttp::Response, String>>;

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

    fn perform(self) -> bevy_jobs::PerformReturn<Self::Outcome> {
        let (sender, receiver): (FetchedFileSender, FetchedFileReceiver) =
            async_channel::unbounded();
        Box::pin(async move {
            fetch(self.url, sender);
            match receiver.recv().await {
                Ok(response) => Ok(FetchedFile {
                    bytes: response?.bytes,
                    crs: self.crs,
                    name: self.name,
                }),
                Err(e) => Err(e.to_string()),
            }
        })
    }
}

fn fetch(url: String, fetched_bytes_sender: FetchedFileSender) {
    let request = ehttp::Request::get(url);
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        if let Err(e) = fetched_bytes_sender.try_send(result) {
            bevy::log::error!("Failed to send network response to main thread: {:?}", e);
        }
    });
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, _app: &mut bevy::app::App) {}
}
