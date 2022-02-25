use bevy::{core_pipeline::ClearColor, prelude::*};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run() {
    let cli_values = rgis_cli::run();
    let source_srs = cli_values.source_srs.clone();
    let target_srs = cli_values.target_srs.clone();

    let mut app = App::new();

    app.insert_resource(Msaa {
        samples: cli_values.msaa_sample_count,
    })
    .add_plugins(DefaultPlugins)
    // Bevy plugins
    // .add_plugin(bevy::log::LogPlugin::default())
    // .add_plugin(bevy::core::CorePlugin::default())
    // .add_plugin(bevy::transform::TransformPlugin::default())
    // .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
    // .add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
    // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    // .add_plugin(bevy::input::InputPlugin::default())
    // .add_plugin(bevy::window::WindowPlugin::default())
    // .add_plugin(bevy::asset::AssetPlugin::default())
    // .add_plugin(bevy::render::RenderPlugin::default())
    //     base_render_graph_config: Some(
    //         bevy::render::render_graph::base::BaseRenderGraphConfig {
    //             // We don’t need a 3D camera
    //             add_3d_camera: false,
    //             ..Default::default()
    //         },
    //     ),
    // })
    // .add_plugin(bevy::sprite::SpritePlugin::default())
    // .add_plugin(bevy::pbr::PbrPlugin::default())
    // .add_plugin(bevy::ui::UiPlugin::default())
    // .add_plugin(bevy::text::TextPlugin::default())
    // .add_plugin(bevy::winit::WinitPlugin::default())
    .add_plugin(rgis_cli::Plugin(cli_values))
    .add_plugin(rgis_layers::RgisLayersPlugin)
    .add_plugin(rgis_file_loader::RgisFileLoaderPlugin)
    .add_plugin(rgis_renderer::RgisRendererPlugin)
    .add_plugin(rgis_mouse::Plugin)
    .add_plugin(rgis_keyboard::Plugin)
    .add_plugin(rgis_camera::RgisCamera)
    .add_plugin(rgis_events::RgisEventsPlugin)
    .add_plugin(rgis_ui::RgisUi {
        source_srs,
        target_srs,
    })
    .insert_resource(ClearColor(Color::WHITE));

    #[cfg(target_arch = "wasm32")]
    {
        app.add_system(bevy_web_resizer::web_resize_system);
    }

    app.run();
}
