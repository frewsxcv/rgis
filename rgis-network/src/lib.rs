pub struct FetchedFile {
    pub name: String,
    pub bytes: Vec<u8>,
    pub crs: String,
}

pub type FetchedFileSender = async_channel::Sender<Result<FetchedFile, String>>;
pub type FetchedFileReceiver = async_channel::Receiver<Result<FetchedFile, String>>;

pub fn fetch(url: String, crs: String, name: String, fetched_bytes_sender: FetchedFileSender) {
    // TODO: this should all happen in a background task
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
        let (sender2, receiver2): (FetchedFileSender, FetchedFileReceiver) =
            async_channel::unbounded();
        app.insert_resource(sender2).insert_resource(receiver2);
    }
}
