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
        window: WindowDescriptor {
            title: "rgis".to_string(),
            ..Default::default()
        },
        ..Default::default()
    });
    app.add_plugin(bevy::winit::WinitPlugin::default());
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
