use bevy::prelude::*;
use std::convert::TryFrom;

pub struct LineStringMeshBuilder {
    vertices: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl LineStringMeshBuilder {
    pub fn new() -> Self {
        // TODO: capacity?
        LineStringMeshBuilder {
            vertices: vec![],
            indices: vec![],
        }
    }

    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_line_string(&mut self, line_string: &geo_types::LineString<f64>) {
        let index_base = self.vertices.len();
        for (i, coord) in line_string.0.iter().enumerate() {
            self.vertices.push([coord.x as f32, coord.y as f32]);
            if i != line_string.0.len() - 1 {
                self.indices.push(u32::try_from(index_base + i).unwrap());
                self.indices
                    .push(u32::try_from(index_base + i + 1).unwrap());
            }
        }
    }

    pub fn build(self) -> Mesh {
        build_mesh_from_vertices(self.vertices, self.indices)
    }
}

fn build_mesh_from_vertices(vertices: Vec<[f32; 2]>, indices: Vec<u32>) -> Mesh {
    let num_vertices = vertices.len();
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::LineList);
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
