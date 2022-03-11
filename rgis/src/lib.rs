use bevy::{core_pipeline::ClearColor, prelude::*};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run() {
    let cli_values = rgis_cli::run();

    let mut app = App::new();

    app.insert_resource(Msaa {
        samples: cli_values.msaa_sample_count,
    })
    .insert_resource(WindowDescriptor {
        title: "rgis".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(rgis_cli::Plugin(cli_values))
    .add_plugin(rgis_layers::Plugin)
    .add_plugin(rgis_file_loader::Plugin)
    .add_plugin(rgis_renderer::Plugin)
    .add_plugin(rgis_mouse::Plugin)
    .add_plugin(rgis_keyboard::Plugin)
    .add_plugin(rgis_camera::Plugin)
    .add_plugin(rgis_events::Plugin)
    .add_plugin(rgis_ui::Plugin)
    .add_plugin(rgis_settings::Plugin)
    .insert_resource(ClearColor(Color::WHITE));

    #[cfg(target_arch = "wasm32")]
    {
        app.add_system(bevy_web_resizer::web_resize_system);
    }

    app.run();
}
