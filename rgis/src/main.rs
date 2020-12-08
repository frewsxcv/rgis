use bevy::{prelude::*, render::pass::ClearColor};
use bevy_prototype_lyon::prelude::*;
use geo_lyon::ToPath;

mod plugins;

// TODO: allow these to be controller at command line
static SOURCE_PROJECTION: &str = "EPSG:4326";
// static SOURCE_PROJECTION: &str = "EPSG:2966";
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

        /////////////
        fn add_mesh(
            polygon: &geo::Polygon<f64>,
            material: Handle<ColorMaterial>,
            meshes: &mut Assets<Mesh>,
            commands: &mut Commands,
        ) {
            let (indices, vertices) = polygon.triangulate_raw();
            println!("{:?}", (indices.len(), vertices.len()));
            let indices = indices
                .into_iter()
                .map(|n| u32::try_from(n).unwrap())
                .collect::<Vec<_>>();
            let vertices = vertices
                .chunks(2)
                .map(|n| [n[0] as f32, n[1] as f32])
                .collect::<Vec<_>>();
            let mesh = build_mesh(indices, vertices);
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
        use geo_earcutr::Triangulate;
        use std::convert::TryFrom;
        let mut i = 0;
        match &layer.projected_geometry.geometry {
            geo::Geometry::GeometryCollection(geometry_collection) => {
                for g in geometry_collection {
                    println!("{}", i);
                    // TODO: combine meshes. each geometrycollection should be one mesh
                    match g {
                        geo::Geometry::Polygon(p) => {
                            add_mesh(p, material.clone(), &mut meshes, commands);
                            i+=1;
                        }
                        geo::Geometry::MultiPolygon(p) => {
                            for polygon in &p.0 {
                            i+=1;
                                add_mesh(polygon, material.clone(), &mut meshes, commands)
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        };
        /////////////

        /*
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
        */

        spawned_events.send(LayerSpawned(event.0));
    }
}

fn build_mesh(indices: Vec<u32>, vertices: Vec<[f32; 2]>) -> Mesh {
    let num_vertices = vertices.len();
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    let mut normals = Vec::new();
    normals.resize(num_vertices, [0.0, 0.0, 0.0]);
    let mut uvs = Vec::new();
    uvs.resize(num_vertices, [0.0, 0.0]);

    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
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
