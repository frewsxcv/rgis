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

#[derive(Component)]
struct SelectedFeature;

#[derive(Component)]
struct PolygonMesh;

#[derive(Component)]
struct LineStringMesh;

fn spawn_geometry_meshes(
    prepared_meshes: Vec<geo_bevy::PreparedMesh>,
    materials: &mut Assets<ColorMaterial>,
    layer: &rgis_layers::Layer,
    commands: &mut Commands,
    assets_meshes: &mut Assets<Mesh>,
    z_index: usize,
    asset_server: &AssetServer,
    is_selected: bool,
) {
    for prepared_mesh in prepared_meshes {
        match prepared_mesh {
            geo_bevy::PreparedMesh::Point(points) => {
                for geo::Point(coord) in points {
                    let mut transform = Transform::from_xyz(coord.x as f32, coord.y as f32, 0.);
                    transform.translation = (coord.x as f32, coord.y as f32, z_index as f32).into();
                    let mut entity_commands =
                        spawn_sprite_bundle(asset_server, transform, commands, layer.color);
                    entity_commands.insert(layer.id);
                    if is_selected {
                        entity_commands.insert(SelectedFeature);
                    }
                }
            }
            geo_bevy::PreparedMesh::Polygon { mesh, color } => {
                spawn_helper(
                    materials,
                    color,
                    is_selected,
                    z_index,
                    mesh,
                    commands,
                    assets_meshes,
                    layer,
                )
                .insert(PolygonMesh);
            }
            geo_bevy::PreparedMesh::LineString { mesh, color } => {
                spawn_helper(
                    materials,
                    color,
                    is_selected,
                    z_index,
                    mesh,
                    commands,
                    assets_meshes,
                    layer,
                )
                .insert(LineStringMesh);
            }
        }
    }
}

fn spawn_helper<'w, 's, 'a>(
    materials: &'a mut Assets<ColorMaterial>,
    color: bevy::render::color::Color,
    is_selected: bool,
    z_index: usize,
    mesh: Mesh,
    commands: &'a mut Commands<'w, 's>,
    assets_meshes: &'a mut Assets<Mesh>,
    layer: &rgis_layers::Layer,
) -> bevy::ecs::system::EntityCommands<'w, 's, 'a> {
    let material = materials.add(color.into());
    let z_index = if is_selected { z_index + 1 } else { z_index };
    let mut entity_commands = spawn_material_mesh_2d_bundle(
        mesh,
        z_index,
        material,
        assets_meshes,
        commands,
        layer.visible,
    );
    entity_commands.insert(layer.id);
    if is_selected {
        entity_commands.insert(SelectedFeature);
    }
    entity_commands
}

fn spawn_sprite_bundle<'w, 's, 'a>(
    asset_server: &AssetServer,
    transform: Transform,
    commands: &'a mut Commands<'w, 's>,
    color: Color,
) -> bevy::ecs::system::EntityCommands<'w, 's, 'a> {
    let bundle = SpriteBundle {
        sprite: Sprite {
            color,
            ..Default::default()
        },
        texture: asset_server.load("circle.png"),
        transform,
        ..Default::default()
    };
    commands.spawn(bundle)
}

fn spawn_material_mesh_2d_bundle<'w, 's, 'a>(
    mesh: Mesh,
    z_index: usize,
    material: Handle<ColorMaterial>,
    assets_meshes: &'a mut Assets<Mesh>,
    commands: &'a mut Commands<'w, 's>,
    is_visible: bool,
) -> bevy::ecs::system::EntityCommands<'w, 's, 'a> {
    let mmb = bevy::sprite::MaterialMesh2dBundle {
        material,
        mesh: bevy::sprite::Mesh2dHandle(assets_meshes.add(mesh)),
        transform: Transform::from_xyz(0., 0., z_index as f32),
        visibility: bevy::render::view::Visibility { is_visible },
        ..Default::default()
    };
    commands.spawn(mmb)
}
