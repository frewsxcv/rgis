#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "rgis".to_string(),
        ..Default::default()
    })
    .add_plugins(MinimalPlugins)
    .add_plugin(bevy::asset::AssetPlugin::default())
    .add_plugin(bevy::window::WindowPlugin::default())
    .add_plugin(bevy::log::LogPlugin::default())
    .add_plugin(bevy::winit::WinitPlugin::default())
    .add_plugin(bevy::input::InputPlugin::default())
    .add_plugin(bevy::render::RenderPlugin::default())
    .add_plugin(bevy::core_pipeline::CorePipelinePlugin::default())
    .add_plugin(bevy::transform::TransformPlugin::default())
    .add_plugin(bevy::sprite::SpritePlugin::default())
    .add_plugin(rgis_layers::Plugin)
    .add_plugin(rgis_file_loader::Plugin)
    .add_plugin(rgis_renderer::Plugin)
    .add_plugin(rgis_mouse::Plugin)
    .add_plugin(rgis_keyboard::Plugin)
    .add_plugin(rgis_network::Plugin)
    .add_plugin(rgis_camera::Plugin)
    .add_plugin(rgis_events::Plugin)
    .add_plugin(bevy_jobs::Plugin)
    .add_plugin(rgis_transform::Plugin)
    .add_plugin(rgis_ui::Plugin)
    .add_plugin(rgis_settings::Plugin)
    .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin);

    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugin(bevy_web_resizer::Plugin);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let cli_values = if let Ok(c) = rgis_cli::run() {
            c
        } else {
            return;
        };
        app.insert_resource(Msaa {
            samples: cli_values.msaa_sample_count,
        })
        .add_plugin(rgis_cli::Plugin(cli_values));
    }

    app.run();
}
