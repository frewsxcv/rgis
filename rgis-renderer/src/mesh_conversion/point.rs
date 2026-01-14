use geo_traits::CoordTrait;
use num_traits::ToPrimitive;

pub struct SpritePosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Default)]
pub struct PointMeshBuilder {
    points: Vec<SpritePosition>,
}

impl PointMeshBuilder {
    /// Call for `add_earcutr_input` for each polygon you want to add to the mesh.
    pub fn add_coord(&mut self, coord: impl CoordTrait) {
        self.points.push(SpritePosition {
            x: coord.x().to_f32().unwrap(),
            y: coord.y().to_f32().unwrap(),
        });
    }
}

impl crate::mesh_conversion::build_mesh::BuildMesh for PointMeshBuilder {
    fn build(self) -> Result<crate::mesh_conversion::GeometryMesh, crate::mesh_conversion::Error> {
        if self.points.is_empty() {
            Err(crate::mesh_conversion::Error::EmptyGeometry)
        } else {
            Ok(crate::mesh_conversion::GeometryMesh::Point(self.points))
        }
    }
}
