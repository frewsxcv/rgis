use crate::Vertex;
use std::error;

pub struct LineStringMeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl LineStringMeshBuilder {
    pub fn new(z_index: usize) -> Self {
        // TODO: capacity?
        LineStringMeshBuilder {
            vertices: vec![],
            indices: vec![],
        }
    }

    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_line_string(
        &mut self,
        line_string: &geo::LineString<f64>,
    ) -> Result<(), Box<dyn error::Error>> {
        let index_base = self.vertices.len();
        for (i, coord) in line_string.0.iter().enumerate() {
            self.vertices.push([coord.x as f32, coord.y as f32, 0.0]);
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
