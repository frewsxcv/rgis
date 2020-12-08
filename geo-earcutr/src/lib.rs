pub trait Triangulate {
    fn traingulate(&self) -> Vec<geo_types::Triangle<f64>>;
}

impl Triangulate for geo_types::Polygon<f64> {
    fn traingulate(&self) -> Vec<geo_types::Triangle<f64>> {
        // TODO: better Vec preallocation
        let mut vertices = vec![];
        let mut interior_indexes = Vec::with_capacity(self.interiors().len());

        vertices.append(&mut flat_line_string_coords(self.exterior()));

        for interior in self.interiors() {
            interior_indexes.push(vertices.len());
            vertices.append(&mut flat_line_string_coords(interior));
        }

        let result = earcutr::earcut(
            &vertices, &interior_indexes, 2
        );

        let mut triangles = vec![];

        for index in result.chunks(3) {
            triangles.push(
                geo_types::Triangle(
                    geo_types::Coordinate {
                        x: vertices[index[0] * 2],
                        y: vertices[index[0] * 2 + 1],
                    },
                    geo_types::Coordinate {
                        x: vertices[index[1] * 2],
                        y: vertices[index[1] * 2 + 1],
                    },
                    geo_types::Coordinate {
                        x: vertices[index[2] * 2],
                        y: vertices[index[2] * 2 + 1],
                    },
                )
            );
        }

        triangles
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
