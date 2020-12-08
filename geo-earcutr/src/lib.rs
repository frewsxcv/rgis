type EarcutrIndices = Vec<usize>;
type EarcutrVertices = Vec<f64>;

pub struct Builder {
    pub indices: EarcutrIndices,
    pub vertices: EarcutrVertices,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            // TODO: better preallocation here
            indices: vec![],
            vertices: vec![],
        }
    }

    pub fn add_geometry<G: Triangulate>(&mut self, geometry: &G) {
        let (mut indices, mut vertices) = geometry.triangulate_raw();
        let index_base = self.vertices.len() / 2;
        for index in &mut indices {
            *index += index_base;
        }
        self.indices.append(&mut indices);
        self.vertices.append(&mut vertices);
    }
}

pub trait Triangulate {
    fn triangulate_raw(&self) -> (Vec<usize>, Vec<f64>);
}

impl Triangulate for geo_types::Polygon<f64> {
    fn triangulate_raw(&self) -> (Vec<usize>, Vec<f64>) {
        // TODO: better Vec preallocation
        let mut vertices = vec![];
        let mut interior_indices = Vec::with_capacity(self.interiors().len());

        vertices.append(&mut flat_line_string_coords(self.exterior()));

        for interior in self.interiors() {
            interior_indices.push(vertices.len() / 2);
            vertices.append(&mut flat_line_string_coords(interior));
        }

        (
            earcutr::earcut(&vertices, &interior_indexes, 2),
            vertices,
        )
    }
}

// TODO: should this return an ExactSizeIterator?
// TODO: actually maybe it should take a &mut [f64] as an arg
fn flat_line_string_coords(line_string: &geo_types::LineString<f64>) -> Vec<f64> {
    let mut v = Vec::with_capacity(line_string.0.len() * 2);
    for coord in line_string.0.iter() {
        v.push(coord.x);
        v.push(coord.y);
    }
    v
}
