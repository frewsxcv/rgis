use bevy::prelude::*;
use bevy::mesh::Indices;

/// Converts a `LineList` mesh (1px hardware lines) into a `TriangleList` mesh
/// where each line segment becomes a quad with the given `width` in world units.
///
/// When `width` is at or below the default (1.0), the original mesh is returned
/// unchanged so that hardware 1px lines are preserved.
pub fn thicken_line_mesh(mesh: Mesh, width: f32) -> Mesh {
    if width <= 1.0 {
        return mesh;
    }

    let Some(positions) = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .and_then(|attr| attr.as_float3())
    else {
        return mesh;
    };

    let Some(Indices::U32(indices)) = mesh.indices() else {
        return mesh;
    };

    let half_width = width / 2.0;
    let segment_count = indices.len() / 2;
    let mut new_positions = Vec::with_capacity(segment_count * 4);
    let mut new_indices = Vec::with_capacity(segment_count * 6);

    for chunk in indices.chunks(2) {
        let a = positions[chunk[0] as usize];
        let b = positions[chunk[1] as usize];

        let dx = b[0] - a[0];
        let dy = b[1] - a[1];
        let len = (dx * dx + dy * dy).sqrt();
        if len < f32::EPSILON {
            continue;
        }

        // Perpendicular offset
        let px = -dy / len * half_width;
        let py = dx / len * half_width;

        let base = new_positions.len() as u32;

        // Quad vertices
        new_positions.push([a[0] - px, a[1] - py, a[2]]);
        new_positions.push([a[0] + px, a[1] + py, a[2]]);
        new_positions.push([b[0] + px, b[1] + py, b[2]]);
        new_positions.push([b[0] - px, b[1] - py, b[2]]);

        // Two triangles
        new_indices.push(base);
        new_indices.push(base + 1);
        new_indices.push(base + 2);
        new_indices.push(base);
        new_indices.push(base + 2);
        new_indices.push(base + 3);
    }

    let mut new_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD,
    );
    new_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
    new_mesh.insert_indices(Indices::U32(new_indices));
    new_mesh
}
