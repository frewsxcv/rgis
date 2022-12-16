pub struct PolygonMeshBuilder {
    polygons: Vec<geo::Polygon>,
}

impl PolygonMeshBuilder {
    pub fn new() -> Self {
        PolygonMeshBuilder {
            polygons: vec![],
        }
    }

    pub fn add_polygon(&mut self, polygon: geo::Polygon) {
    }

    pub fn build(self) -> Option<crate::PreparedMesh> {
        todo!()
    }
}
