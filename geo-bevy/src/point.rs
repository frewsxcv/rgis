use crate::Vertex;
use bevy_render::prelude::*;
use std::num;

pub struct PointMeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl PointMeshBuilder {
    pub fn new() -> Self {
        PointMeshBuilder {
            vertices: vec![],
            indices: vec![],
        }
    }

    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_point(&mut self, point: &geo::Point) -> Result<(), num::TryFromIntError> {
        let index_base = self.vertices.len();
        self.vertices
            .push([point.x() as f32, point.y() as f32, 0.0f32]);
        self.indices.push(u32::try_from(index_base)?);
        Ok(())
    }

    pub fn build(self) -> Option<crate::PreparedMesh> {
        if self.vertices.is_empty() {
            None
        } else {
            let mut mesh: Mesh = shape::Box::new(1.0, 1.0, 0.0).into();
            let num_vertices = self.vertices.len();
            mesh.set_indices(Some(bevy_render::mesh::Indices::U32(self.indices)));
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
            let normals = vec![[0.0, 0.0, 0.0]; num_vertices];
            let uvs = vec![[0.0, 0.0]; num_vertices];

            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

            Some(crate::PreparedMesh::Point { mesh })
        }
    }
}
