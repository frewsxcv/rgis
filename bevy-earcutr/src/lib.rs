use bevy::prelude::*;
use std::convert::TryFrom;

type EarcutrIndices = Vec<usize>;
type EarcutrVertices = Vec<f64>;
type BevyIndices = Vec<u32>;
type BevyVertices = Vec<[f32; 2]>;

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

pub fn build_mesh_from_earcutr(indices: EarcutrIndices, vertices: EarcutrVertices) -> Mesh {
    let indices = indices
        .into_iter()
        .map(|n| u32::try_from(n).unwrap())
        .collect::<Vec<_>>();
    let vertices = vertices
        .chunks(2)
        .map(|n| [n[0] as f32, n[1] as f32])
        .collect::<Vec<_>>();
    build_mesh_from_bevy(indices, vertices)
}

fn build_mesh_from_bevy(indices: BevyIndices, vertices: BevyVertices) -> Mesh {
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
