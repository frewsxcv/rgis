#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

pub struct FetchedFile {
    pub name: String,
    pub bytes: Vec<u8>,
    pub crs: String,
}

type FetchedFileSender = async_channel::Sender<Result<FetchedFile, String>>;
type FetchedFileReceiver = async_channel::Receiver<Result<FetchedFile, String>>;

pub struct NetworkFetchTask {
    pub url: String,
    pub crs: String,
    pub name: String,
}

impl rgis_task::Task for NetworkFetchTask {
    type Outcome = Result<FetchedFile, String>;

    fn name(&self) -> String {
        format!("Fetching '{}'", self.name)
    }

    fn perform(self) -> rgis_task::PerformReturn<Self::Outcome> {
        let (sender, receiver): (FetchedFileSender, FetchedFileReceiver) =
            async_channel::unbounded();
        Box::pin(async move {
            fetch(self.url, self.crs, self.name, sender);
            match receiver.recv().await {
                Ok(n) => n,
                Err(e) => Err(e.to_string()),
            }
        })
    }
}

fn fetch(url: String, crs: String, name: String, fetched_bytes_sender: FetchedFileSender) {
    let request = ehttp::Request::get(url);
    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        if let Err(e) = fetched_bytes_sender.try_send(result.map(|r| FetchedFile {
            bytes: r.bytes,
            crs,
            name,
        })) {
            bevy::log::error!("Failed to send network response to main thread: {:?}", e);
        }
    });
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugin(rgis_task::TaskPlugin::<NetworkFetchTask>::new());
    }
}
