#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;

mod systems;
mod tasks;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(systems::system_set());
    }
}

fn spawn_geometry_meshes(
    prepared_meshes: Vec<geo_bevy::PreparedMesh>,
    materials: &mut Assets<ColorMaterial>,
    layer: &rgis_layers::Layer,
    commands: &mut Commands,
    assets_meshes: &mut Assets<Mesh>,
    z_index: usize,
    is_visible: bool,
    asset_server: &AssetServer,
) {
    for prepared_mesh in prepared_meshes {
        match prepared_mesh {
            geo_bevy::PreparedMesh::Point(points) => {
                for geo::Point(coord) in points {
                    let mut transform = Transform::from_xyz(coord.x as f32, coord.y as f32, 0.);
                    transform.translation = (coord.x as f32, coord.y as f32, z_index as f32).into();
                    spawn_sprite_bundle(asset_server, transform, commands, layer.id, layer.color);
                }
            }
            geo_bevy::PreparedMesh::PolygonAndLineString { mesh, color } => {
                let material = materials.add(color.into());
                spawn_material_mesh_2d_bundle(
                    mesh,
                    z_index,
                    material,
                    assets_meshes,
                    commands,
                    layer.id,
                    is_visible,
                );
            }
        }
    }
}

fn spawn_sprite_bundle(
    asset_server: &AssetServer,
    transform: Transform,
    commands: &mut Commands,
    layer_id: rgis_layer_id::LayerId,
    color: Color,
) {
    let bundle = SpriteBundle {
        sprite: Sprite {
            color,
            ..Default::default()
        },
        texture: asset_server.load("circle.png"),
        transform,
        ..Default::default()
    };
    commands.spawn_bundle(bundle).insert(layer_id);
}

fn spawn_material_mesh_2d_bundle(
    mesh: Mesh,
    z_index: usize,
    material: Handle<ColorMaterial>,
    assets_meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
    layer_id: rgis_layer_id::LayerId,
    is_visible: bool,
) {
    let mmb = bevy::sprite::MaterialMesh2dBundle {
        material,
        mesh: bevy::sprite::Mesh2dHandle(assets_meshes.add(mesh)),
        transform: Transform::from_xyz(0., 0., z_index as f32),
        visibility: bevy::render::view::Visibility { is_visible },
        ..Default::default()
    };
    commands.spawn_bundle(mmb).insert(layer_id);
}
