use bevy::{app::Events, core_pipeline::ClearColor, prelude::*};
use geo_bevy::BuildBevyMeshes;

// System
fn layer_loaded(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: rgis_layers::ResLayers,
    mut event_reader: EventReader<rgis_layers::LayerLoaded>,
    mut spawned_events: ResMut<Events<rgis_layers::LayerSpawned>>,
) {
    for event in event_reader.iter() {
        let layers = layers.read().unwrap();
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };
        let material = materials.add(layer.color.into());

        let tl = time_logger::start(&format!("Triangulating and building {} mesh", layer.name));
        for mesh in layer
            .projected_geometry
            .geometry
            .build_bevy_meshes(geo_bevy::BuildBevyMeshesContext::new())
        {
            spawn_mesh(mesh, material.clone(), &mut meshes, &mut commands);
        }
        tl.finish();

        spawned_events.send(rgis_layers::LayerSpawned(event.0));
    }
}

struct LayerSpawnedPlugin;

impl Plugin for LayerSpawnedPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(layer_spawned.system());
    }
}

// System
fn layer_spawned(
    layers: rgis_layers::ResLayers,
    mut camera_offset: ResMut<rgis_camera::CameraOffset>,
    mut camera_scale: ResMut<rgis_camera::CameraScale>,
    mut event_reader: EventReader<rgis_layers::LayerSpawned>,
) {
    for event in event_reader.iter() {
        let layers = layers.read().unwrap();
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };
        let layer_center = layer.projected_bounding_rect.rect.center();
        // TODO: this scale math is inprecise. it should take into account
        // .     the height of the geometry. as well as the window size.
        let scale = layer.projected_bounding_rect.rect.width() / 1_000.;
        // TODO: only change the transform if there were no layers previously
        debug!("Moving camera to look at new layer");
        camera_offset.x = layer_center.x as f32;
        camera_offset.y = layer_center.y as f32;
        camera_scale.0 = scale as f32;
    }
}

pub fn spawn_mesh(
    mesh: Mesh,
    material: Handle<ColorMaterial>,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
) {
    let mmb = bevy::sprite::MaterialMesh2dBundle {
        material,
        mesh: bevy::sprite::Mesh2dHandle(meshes.add(mesh)),
        ..Default::default()
    };
    commands.spawn_bundle(mmb);
}

fn main() {
    let cli_values = rgis_cli::run();
    let source_srs = cli_values.source_srs.clone();
    let target_srs = cli_values.target_srs.clone();

    App::new()
        .insert_resource(Msaa {
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
        .add_system(layer_loaded.system())
        .add_plugin(rgis_mouse::Plugin)
        .add_plugin(rgis_keyboard::Plugin)
        .add_plugin(LayerSpawnedPlugin)
        .add_plugin(rgis_camera::RgisCamera)
        .add_plugin(rgis_ui::RgisUi {
            source_srs,
            target_srs,
        })
        .insert_resource(ClearColor(Color::WHITE))
        .run();
}
