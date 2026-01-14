use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::{Fill, RenderEntityType, ZIndex};

pub fn spawn_fill_helper(
    materials: &mut Assets<ColorMaterial>,
    color: Color,
    layer_index: crate::rgis_layers::LayerIndex,
    mesh: Mesh,
    commands: &mut RelatedSpawnerCommands,
    assets_meshes: &mut Assets<Mesh>,
    entity_type: RenderEntityType,
) {
    let material = materials.add(color);
    let z_index = ZIndex::calculate(layer_index, entity_type);
    commands.spawn((
        MeshBundle {
            mesh: assets_meshes.add(mesh),
            transform: Transform::from_xyz(0., 0., z_index.0 as f32),
            material,
            ..Default::default()
        },
        entity_type,
        Fill,
    ));
}
