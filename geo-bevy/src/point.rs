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
            Some(crate::PreparedMesh::Point)
        }
    }
}
