use crate::window::UserEvent;
use glutin::event_loop::EventLoopProxy;

mod geojson;

pub fn load(file_path: String, event_loop_proxy: EventLoopProxy<UserEvent>) {
    let e = event_loop_proxy.clone();
    log::info!("Spawning a new thread for loading: {}", file_path);
    rayon::spawn(move || {
        geojson::load(file_path);
        e.send_event(UserEvent::Render).unwrap();
    })
}
