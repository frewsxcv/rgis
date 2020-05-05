use crate::layer::Layers;
use crate::window::UserEvent;
use glutin::event_loop::EventLoopProxy;
use std::sync;

mod geojson;

pub fn load(
    file_path: String,
    event_loop_proxy: EventLoopProxy<UserEvent>,
    layers: sync::Arc<sync::RwLock<Layers>>,
) {
    let e = event_loop_proxy.clone();
    log::info!("Spawning a new thread for loading: {}", file_path);
    rayon::spawn(move || {
        let count = geojson::load(file_path, layers);
        if count > 0 {
            e.send_event(UserEvent::LayerAdded).unwrap();
        }
    })
}
