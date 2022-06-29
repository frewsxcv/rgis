#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
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
    layer_id: rgis_layer_id::LayerId,
    commands: &mut Commands,
    assets_meshes: &mut Assets<Mesh>,
    z_index: usize,
    is_visible: bool,
) {
    for prepared_mesh in prepared_meshes {
        let material = materials.add(prepared_mesh.color.into());

        spawn_mesh(
            prepared_mesh.mesh,
            z_index,
            material.clone(),
            assets_meshes,
            commands,
            layer_id,
            is_visible,
        );
    }
}

fn spawn_mesh(
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
