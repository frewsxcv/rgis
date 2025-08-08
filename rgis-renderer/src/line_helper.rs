use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::{line_material::LineMaterial, RenderEntityType, Stroke, ZIndex};

pub fn spawn_line_string_helper(
    materials: &mut Assets<LineMaterial>,
    color: Color,
    width: f32,
    layer_index: crate::rgis_layers::LayerIndex,
    mesh: Mesh,
    commands: &mut RelatedSpawnerCommands,
    assets_meshes: &mut Assets<Mesh>,
    entity_type: RenderEntityType,
) {
    let material = materials.add(LineMaterial { color, width });
    let z_index = ZIndex::calculate(layer_index, entity_type);
    commands.spawn((
        MeshBundle {
            mesh: assets_meshes.add(mesh),
            transform: Transform::from_xyz(0., 0., z_index.0 as f32),
            material,
            ..Default::default()
        },
        entity_type,
        Stroke,
    ));
}
