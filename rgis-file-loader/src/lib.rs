use rgis_layers::Layers;
use std::sync;

mod geojson;

pub fn load<F: FnOnce() + Send + 'static>(
    file_path: String,
    layers: sync::Arc<sync::RwLock<Layers>>,
    source_projection: &'static str,
    target_projection: &'static str,
    callback_on_layer_add: F,
) {
    log::info!("Spawning a new thread for loading: {}", file_path);
    rayon::spawn(move || {
        let count = geojson::load(file_path, layers, source_projection, target_projection);
        if count > 0 {
            callback_on_layer_add();
        }
    })
}
