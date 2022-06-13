use crate::Vertex;
use std::num;

pub struct LineStringMeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl LineStringMeshBuilder {
    pub fn new() -> Self {
        LineStringMeshBuilder {
            vertices: vec![],
            indices: vec![],
        }
    }

    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_line_string(
        &mut self,
        line_string: &geo::LineString<f64>,
    ) -> Result<(), num::TryFromIntError> {
        let index_base = self.vertices.len();

        self.vertices.reserve(self.vertices.len());
        self.indices.reserve(self.indices.len() * 2);

        for (i, coord) in line_string.0.iter().enumerate() {
            self.vertices.push([coord.x as f32, coord.y as f32, 0.0f32]);
            if i != line_string.0.len() - 1 {
                self.indices.push(u32::try_from(index_base + i)?);
                self.indices.push(u32::try_from(index_base + i + 1)?);
            }
        }
        Ok(())
    }

    pub fn build(self) -> Option<bevy_render::prelude::Mesh> {
        if self.vertices.is_empty() {
            None
        } else {
            Some(crate::build_mesh_from_vertices(
                bevy_render::render_resource::PrimitiveTopology::LineList,
                self.vertices,
                self.indices,
            ))
        }
    }
}
