use crate::Vertex;
use std::error;

pub struct PointMeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl PointMeshBuilder {
    pub fn new() -> Self {
        // TODO: capacity?
        PointMeshBuilder {
            vertices: vec![],
            indices: vec![],
        }
    }

    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_point(&mut self, point: &geo::Point<f64>) -> Result<(), Box<dyn error::Error>> {
        let index_base = self.vertices.len();
        self.vertices
            .push([point.x() as f32, point.y() as f32, 0.0f32]);
        self.indices.push(u32::try_from(index_base)?);
        Ok(())
    }

    pub fn build(self) -> Option<bevy_render::prelude::Mesh> {
        if self.vertices.is_empty() {
            None
        } else {
            Some(crate::build_mesh_from_vertices(
                bevy_render::render_resource::PrimitiveTopology::PointList,
                self.vertices,
                self.indices,
            ))
        }
    }
}
