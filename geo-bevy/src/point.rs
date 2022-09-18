use std::num;

pub struct PointMeshBuilder {
    points: Vec<geo::Point>,
}

impl PointMeshBuilder {
    pub fn new() -> Self {
        PointMeshBuilder { points: vec![] }
    }

    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_point(&mut self, point: &geo::Point) -> Result<(), num::TryFromIntError> {
        self.points.push(*point);
        Ok(())
    }

    pub fn build(self) -> Option<crate::PreparedMesh> {
        if self.points.is_empty() {
            None
        } else {
            Some(crate::PreparedMesh::Point(self.points))
        }
    }
}
