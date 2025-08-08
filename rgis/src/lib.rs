use bevy::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::asset::AssetPlugin::default());
    app.add_plugins(WindowPlugin {
        primary_window: Some(Window {
            title: "rgis".to_string(),
            canvas: Some("#rgis".into()), // selector
            ..Default::default()
        }),
        ..Default::default()
    });
    app.add_plugins(bevy::a11y::AccessibilityPlugin);
    app.add_plugins(bevy::winit::WinitPlugin::<bevy::winit::WakeUp>::default());
    app.add_plugins(bevy::render::RenderPlugin::default());
    app.add_plugins(bevy::render::texture::ImagePlugin::default());
    app.add_plugins(bevy::log::LogPlugin::default());
    app.add_plugins(bevy::input::InputPlugin);
    app.add_plugins(bevy::core_pipeline::CorePipelinePlugin);
    app.add_plugins(bevy::transform::TransformPlugin);
    app.add_plugins(bevy::sprite::SpritePlugin);
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
    app.add_plugins(bevy::diagnostic::DiagnosticsPlugin);
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());

    app.run();
}
