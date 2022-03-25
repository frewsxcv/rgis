use bevy::app::Events;
use bevy::prelude::*;

mod geojson;

struct SpawnedLayers(Vec<rgis_layers::UnassignedLayer>);

type LoadedGeoJsonFileSender = async_channel::Sender<SpawnedLayers>;
type LoadedGeoJsonFileReceiver = async_channel::Receiver<SpawnedLayers>;

struct FetchedFile {
    name: String,
    bytes: Vec<u8>,
    crs: String,
}

type FetchedFileSender = async_channel::Sender<FetchedFile>;
type FetchedFileReceiver = async_channel::Receiver<FetchedFile>;

// System
fn load_geojson_file_handler(
    mut load_event_reader: ResMut<Events<rgis_events::LoadGeoJsonFileEvent>>,
    thread_pool: Res<bevy::tasks::AsyncComputeTaskPool>,
    sender: Res<LoadedGeoJsonFileSender>,
    rgis_settings: Res<rgis_settings::RgisSettings>,
    fetched_bytes_sender: Res<FetchedFileSender>,
    fetched_bytes_receiver: Res<FetchedFileReceiver>,
) {
    while let Ok(fetched) = fetched_bytes_receiver.try_recv() {
        load_event_reader.send(rgis_events::LoadGeoJsonFileEvent::FromBytes {
            bytes: fetched.bytes,
            file_name: fetched.name,
            crs: fetched.crs,
        })
    }
    for event in load_event_reader.drain() {
        let sender: LoadedGeoJsonFileSender = sender.clone();
        let target_crs = rgis_settings.target_crs.clone();
        let fetched_bytes_sender = fetched_bytes_sender.clone();
        thread_pool
            .spawn(async move {
                match event {
                    #[cfg(not(target_arch = "wasm32"))]
                    rgis_events::LoadGeoJsonFileEvent::FromPath {
                        path: geojson_file_path,
                        crs,
                    } => {
                        let spawned_layers = SpawnedLayers(geojson::load_from_path(
                            geojson_file_path,
                            &crs,
                            &target_crs,
                        ));
                        sender.send(spawned_layers).await.unwrap();
                    }
                    rgis_events::LoadGeoJsonFileEvent::FromNetwork { url, crs, name } => {
                        let request = ehttp::Request::get(url);
                        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
                            let bytes = result.unwrap().bytes;
                            fetched_bytes_sender
                                .try_send(FetchedFile { bytes, crs, name })
                                .unwrap();
                        });
                    }
                    rgis_events::LoadGeoJsonFileEvent::FromBytes {
                        file_name,
                        bytes,
                        crs,
                    } => {
                        let spawned_layers = SpawnedLayers(geojson::load_from_reader(
                            std::io::Cursor::new(bytes),
                            file_name,
                            &crs,
                            &target_crs,
                        ));
                        sender.send(spawned_layers).await.unwrap();
                    }
                }
            })
            .detach();
    }
}

fn handle_loaded_layers(
    mut loaded_events: EventWriter<rgis_events::LayerLoadedEvent>,
    mut layers: ResMut<rgis_layers::Layers>,
    receiver: Res<LoadedGeoJsonFileReceiver>,
) {
    while let Ok(spawned_layers) = receiver.try_recv() {
        for unassigned_layer in spawned_layers.0 {
            let layer_id = layers.add(unassigned_layer);
            loaded_events.send(rgis_events::LayerLoadedEvent(layer_id));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_layers_from_cli(
    cli_values: Res<rgis_cli::Values>,
    mut events: EventWriter<rgis_events::LoadGeoJsonFileEvent>,
) {
    for geojson_file_path in &cli_values.geojson_files {
        debug!(
            "sending LoadGeoJsonFile event: {}",
            &geojson_file_path.display(),
        );
        events.send(rgis_events::LoadGeoJsonFileEvent::FromPath {
            path: geojson_file_path.clone(),
            crs: cli_values.source_crs.clone(),
        });
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver): (LoadedGeoJsonFileSender, LoadedGeoJsonFileReceiver) =
            async_channel::unbounded();
        let (sender2, receiver2): (FetchedFileSender, FetchedFileReceiver) =
            async_channel::unbounded();
        app.insert_resource(sender)
            .insert_resource(receiver)
            .insert_resource(sender2)
            .insert_resource(receiver2)
            .add_system(load_geojson_file_handler.system())
            .add_system(handle_loaded_layers.system());

        #[cfg(not(target_arch = "wasm32"))]
        app.add_startup_system(load_layers_from_cli.system());
    }
}
