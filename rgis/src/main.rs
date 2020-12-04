use bevy::{prelude::*, render::pass::ClearColor};
use bevy_prototype_lyon::prelude::*;

mod plugins;

// TODO: allow these to be controller at command line
static SOURCE_PROJECTION: &str = "EPSG:4326";
static TARGET_PROJECTION: &str = "EPSG:3857";

// System
fn load_layers_from_cli(mut events: ResMut<Events<LoadGeoJsonFile>>) {
    for geojson_file_path in rgis_cli::run().unwrap() {
        for _ in 0..2 {
            log::debug!(
                "sending LoadGeoJsonFile event: {}",
                geojson_file_path.clone()
            );
            events.send(LoadGeoJsonFile {
                path: geojson_file_path.clone(),
            });
        }
    }
}

// System
fn load_geojson_file_handler(
    mut layers: ResMut<rgis_layers::Layers>,
    load_events: Res<Events<LoadGeoJsonFile>>,
    mut load_event_reader: Local<EventReader<LoadGeoJsonFile>>,
    mut loaded_events: ResMut<Events<LayerLoaded>>,
) {
    for LoadGeoJsonFile {
        path: geojson_file_path,
    } in load_event_reader.iter(&load_events)
    {
        println!("loading {}", geojson_file_path);
        let count = rgis_file_loader::load(
            geojson_file_path.clone(),
            &mut layers,
            SOURCE_PROJECTION,
            TARGET_PROJECTION,
        );
        // TODO: don't assume layer is the last one
        let layer = layers.data.last().unwrap();
        if count > 0 {
            loaded_events.send(LayerLoaded(layer.id));
        }
    }
}

// System
fn layer_loaded(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: ResMut<rgis_layers::Layers>,
    events: Res<Events<LayerLoaded>>,
    mut event_reader: Local<EventReader<LayerLoaded>>,
    mut spawned_events: ResMut<Events<LayerSpawned>>,
) {
    for event in event_reader.iter(&events) {
        // TODO: find the layer we loaded instead of assuming the first
        let layer = &layers.data[0];
        // TODO: dont assume it is this color
        let material = materials.add(Color::rgb(0.8, 0.0, 0.0).into());
        // TODO: dont assume it's a polygon
        let polygon = match layer.projected_geometry.geometry {
            geo::Geometry::Polygon(ref p) => p,
            _ => unimplemented!(),
        };

        println!("Building sprite from geometry");
        let sprite_components = geo_lyon::convert(polygon).fill(
            material.clone(),
            &mut meshes,
            Vec3::new(0.0, 0.0, 0.0).into(),
            &FillOptions::default(),
        );

        println!("Spawning geometry entity");
        commands.spawn(sprite_components);

        spawned_events.send(LayerSpawned(event.0));
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
    events: Res<Events<LayerSpawned>>,
    layers: ResMut<rgis_layers::Layers>,
    mut event_reader: Local<EventReader<LayerSpawned>>,
    camera_query: Query<(&crate::Camera,)>,
    mut transform_query: Query<(&mut Transform,)>,
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
        for (camera,) in camera_query.iter() {
            if let Ok((mut transform,)) = transform_query.get_mut(camera.0) {
                println!("Moving camera to look at new layer");
                transform.translation =
                    Vec3::new(layer_center.x as f32, layer_center.y as f32, 0.0);
                transform.scale = Vec3::new(scale as f32, scale as f32, 1.0);
            }
        }
    }
}

#[derive(Debug)]
struct LoadGeoJsonFile {
    path: String,
}

#[derive(Debug)]
struct LayerLoaded(rgis_layers::LayerId);

#[derive(Debug)]
struct LayerSpawned(rgis_layers::LayerId);

#[derive(Debug)]
struct Camera(Entity);

fn setup(mut commands: Commands) {
    let entity = commands
        .spawn(Camera2dComponents::default())
        .current_entity();

    commands.spawn((Camera(entity.expect("could not find entity")),));
}

fn main() {
    env_logger::init();

    App::build()
        .add_event::<LoadGeoJsonFile>()
        .add_event::<LayerLoaded>()
        .add_event::<LayerSpawned>()
        .add_plugins(DefaultPlugins)
        .add_resource(rgis_layers::Layers::new())
        .add_startup_system(load_layers_from_cli.system())
        .add_startup_system(setup.system())
        .add_system(load_geojson_file_handler.system())
        .add_system(layer_loaded.system())
        .add_plugin(plugins::KeyboardCameraMover)
        .add_plugin(LayerSpawnedPlugin)
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .run();
}
