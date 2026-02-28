use bevy::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "rgis".to_string(),
                    canvas: Some("#rgis".into()),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .disable::<bevy::log::LogPlugin>(),
    );
    app.add_plugins(bevy::log::LogPlugin::default());
    app.add_plugins(rgis_ui::Plugin);
    app.add_plugins(rgis_layers::Plugin);
    app.add_plugins(rgis_file_loader::Plugin);
    app.add_plugins(rgis_renderer::Plugin);
    app.add_plugins(rgis_mouse::Plugin);
    app.add_plugins(rgis_keyboard::Plugin);
    app.add_plugins(rgis_camera::Plugin);
    app.add_plugins(rgis_ui_events::Plugin);
    app.add_plugins(rgis_camera_events::Plugin);
    app.add_plugins(rgis_layer_events::Plugin);
    app.add_plugins(rgis_map_events::Plugin);
    app.add_plugins(rgis_file_loader_events::Plugin);
    app.add_plugins(rgis_crs_events::Plugin);
    app.add_plugins(rgis_renderer_events::Plugin);
    app.add_plugins(bevy_jobs::Plugin);
    app.add_plugins(rgis_transform::Plugin);
    app.add_plugins(rgis_settings::Plugin);
    app.add_plugins(rgis_geodesy::Plugin);
    app.add_plugins(rgis_crs::Plugin);

    app.run();
}
