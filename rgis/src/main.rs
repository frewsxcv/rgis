use bevy::{prelude::*, render::pass::ClearColor};
use bevy_prototype_lyon::prelude::*;
use geo_lyon::ToPath;

mod plugins;

// TODO: allow these to be controller at command line
static SOURCE_PROJECTION: &str = "EPSG:4326";
static TARGET_PROJECTION: &str = "EPSG:3857";

// System
fn load_layers_from_cli(mut events: ResMut<Events<LoadGeoJsonFile>>) {
    for geojson_file_path in rgis_cli::run().unwrap() {
        log::debug!("sending LoadGeoJsonFile event: {}", &geojson_file_path,);
        events.send(LoadGeoJsonFile {
            path: geojson_file_path,
        });
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
        let layer_ids = rgis_file_loader::load(
            geojson_file_path.clone(),
            &mut layers,
            SOURCE_PROJECTION,
            TARGET_PROJECTION,
        );
        for layer_id in layer_ids {
            loaded_events.send(LayerLoaded(layer_id));
        }
    }
}

// System
fn layer_loaded(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    layers: Res<rgis_layers::Layers>,
    events: Res<Events<LayerLoaded>>,
    mut event_reader: Local<EventReader<LayerLoaded>>,
    mut spawned_events: ResMut<Events<LayerSpawned>>,
    camera_scale: Res<plugins::rgis_camera::CameraScale>,
) {
    for event in event_reader.iter(&events) {
        let layer = match layers.get(event.0) {
            Some(l) => l,
            None => continue,
        };
        let material =
            materials.add(Color::rgb_u8(layer.color.0, layer.color.1, layer.color.2).into());

        log::info!("Building Path for new layer");
        let path = match layer.projected_geometry.geometry {
            geo::Geometry::Polygon(ref g) => g.to_path(),
            geo::Geometry::MultiPolygon(ref g) => g.to_path(),
            geo::Geometry::GeometryCollection(ref g) => g.to_path(),
            geo::Geometry::Triangle(ref g) => g.to_path(),
            _ => {
                log::error!("Encountered a Geometry type we can’t render yet");
                continue;
            },
        };

        println!("Building Sprite from Path");
        let sprite_components = path.fill(
            material.clone(),
            &mut meshes,
            Vec3::new(0.0, 0.0, 0.0).into(),
            &FillOptions::default(),
        );

        log::debug!("Spawning geometry fill entity");
        commands.spawn(sprite_components);

        let material = materials.add(Color::BLACK.into());
        let sprite_components = path.stroke(
            material.clone(),
            &mut meshes,
            Vec3::new(0.0, 0.0, 0.0).into(),
            // FIXME: line width is not being calculated correctly here
            &StrokeOptions::default().with_line_width(1000. * camera_scale.0),
        );

        log::debug!("Spawning geometry stroke entity");
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
    mut camera_offset: ResMut<plugins::rgis_camera::CameraOffset>,
    mut camera_scale: ResMut<plugins::rgis_camera::CameraScale>,
    mut event_reader: Local<EventReader<LayerSpawned>>,
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

#[derive(Debug)]
struct LoadGeoJsonFile {
    path: String,
}

#[derive(Debug)]
struct LayerLoaded(rgis_layers::LayerId);

#[derive(Debug)]
struct LayerSpawned(rgis_layers::LayerId);

fn main() {
    env_logger::init();

    App::build()
        .add_event::<LoadGeoJsonFile>()
        .add_event::<LayerLoaded>()
        .add_event::<LayerSpawned>()
        .add_plugins(DefaultPlugins)
        .add_resource(rgis_layers::Layers::new())
        .add_startup_system(load_layers_from_cli.system())
        .add_system(load_geojson_file_handler.system())
        .add_system(layer_loaded.system())
        .add_plugin(LayerSpawnedPlugin)
        .add_plugin(plugins::rgis_camera::RgisCamera)
        .add_resource(ClearColor(Color::WHITE))
        .run();
}
