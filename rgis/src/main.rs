use bevy::{prelude::*, render::pass::ClearColor};
use geo_bevy::BuildBevyMeshes;

mod mouse;

// System
fn load_layers_from_cli(mut events: ResMut<Events<rgis_file_loader::LoadGeoJsonFile>>) {
    let cli_values = rgis_cli::run();
    for geojson_file_path in cli_values.geojson_files {
        log::debug!("sending LoadGeoJsonFile event: {}", &geojson_file_path,);
        events.send(rgis_file_loader::LoadGeoJsonFile {
            path: geojson_file_path,
            source_srs: cli_values.source_srs.clone(),
            target_srs: cli_values.target_srs.clone(),
        });
    }
}

// System
fn layer_loaded(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: Res<rgis_layers::Layers>,
    events: Res<Events<rgis_layers::LayerLoaded>>,
    mut event_reader: Local<EventReader<rgis_layers::LayerLoaded>>,
    mut spawned_events: ResMut<Events<rgis_layers::LayerSpawned>>,
) {
    for event in event_reader.iter(&events) {
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };
        let material =
            materials.add(Color::rgb_u8(layer.color.0, layer.color.1, layer.color.2).into());

        let tl = time_logger::start("Triangulating and building mesh");
        for mesh in layer
            .projected_geometry
            .geometry
            .build_bevy_meshes(geo_bevy::BuildBevyMeshesContext::new())
        {
            spawn_mesh(mesh, material.clone(), &mut meshes, commands);
        }
        tl.finish();

        spawned_events.send(rgis_layers::LayerSpawned(event.0));
    }
}

struct LayerSpawnedPlugin;

impl Plugin for LayerSpawnedPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(layer_spawned.system());
    }
}

// System
fn layer_spawned(
    events: Res<Events<rgis_layers::LayerSpawned>>,
    layers: ResMut<rgis_layers::Layers>,
    mut camera_offset: ResMut<rgis_camera::CameraOffset>,
    mut camera_scale: ResMut<rgis_camera::CameraScale>,
    mut event_reader: Local<EventReader<rgis_layers::LayerSpawned>>,
) {
    for event in event_reader.iter(&events) {
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };
        let layer_center = layer.projected_bounding_rect.rect.center();
        // TODO: this scale math is inprecise. it should take into account
        // .     the height of the geometry. as well as the window size.
        let scale = layer.projected_bounding_rect.rect.width() / 1_000.;
        // TODO: only change the transform if there were no layers previously
        log::debug!("Moving camera to look at new layer");
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
    let sprite = SpriteBundle {
        material: material,
        mesh: meshes.add(mesh),
        sprite: Sprite {
            size: Vec2::new(1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn(sprite);
}

fn main() {
    env_logger::init();

    App::build()
        .add_event::<rgis_file_loader::LoadGeoJsonFile>()
        .add_event::<rgis_layers::LayerLoaded>()
        .add_event::<rgis_layers::LayerSpawned>()
        .add_plugins(DefaultPlugins)
        .add_resource(rgis_layers::Layers::new())
        .add_startup_system(load_layers_from_cli.system())
        .add_system(rgis_file_loader::load_geojson_file_handler.system())
        .add_system(layer_loaded.system())
        .add_system(mouse::system.system())
        .add_plugin(LayerSpawnedPlugin)
        .add_plugin(rgis_camera::RgisCamera)
        .add_plugin(rgis_ui::RgisUi)
        .add_resource(ClearColor(Color::WHITE))
        .run();
}
