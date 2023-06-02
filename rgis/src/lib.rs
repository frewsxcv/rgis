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

    app.add_plugins(MinimalPlugins);
    app.add_plugin(bevy::asset::AssetPlugin::default());
    app.add_plugin(WindowPlugin {
        primary_window: Some(Window {
            title: "rgis".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    });
    app.add_plugin(bevy::winit::WinitPlugin::default());
    app.add_plugin(bevy::a11y::AccessibilityPlugin);
    app.add_plugin(bevy::render::RenderPlugin::default());
    app.add_plugin(bevy::render::texture::ImagePlugin::default());
    app.add_plugin(bevy::log::LogPlugin::default());
    app.add_plugin(bevy::input::InputPlugin::default());
    app.add_plugin(bevy::core_pipeline::CorePipelinePlugin::default());
    app.add_plugin(bevy::transform::TransformPlugin::default());
    app.add_plugin(bevy::sprite::SpritePlugin::default());
    app.add_plugin(rgis_ui::Plugin);
    app.add_plugin(rgis_layers::Plugin);
    app.add_plugin(rgis_file_loader::Plugin);
    app.add_plugin(rgis_renderer::Plugin);
    app.add_plugin(rgis_mouse::Plugin);
    app.add_plugin(rgis_keyboard::Plugin);
    app.add_plugin(rgis_network::Plugin);
    app.add_plugin(rgis_camera::Plugin);
    app.add_plugin(rgis_events::Plugin);
    app.add_plugin(bevy_jobs::Plugin);
    app.add_plugin(rgis_transform::Plugin);
    app.add_plugin(rgis_settings::Plugin);
    app.add_plugin(bevy::diagnostic::DiagnosticsPlugin);
    app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin);

    #[cfg(not(target_arch = "wasm32"))]
    {
        let cli_values = if let Ok(c) = rgis_cli::run() {
            c
        } else {
            return;
        };
        let msaa = match cli_values.msaa_sample_count {
            1 => Msaa::Off,
            2 => Msaa::Sample2,
            4 => Msaa::Sample4,
            8 => Msaa::Sample8,
            _ => panic!("Encountered unknown MSAA value"),
        };

        app.insert_resource(msaa);
        app.add_plugin(rgis_cli::Plugin(cli_values));
    }

    app.run();
}
