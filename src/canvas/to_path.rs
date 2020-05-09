use pathfinder_geometry::vector::{vec2f, Vector2F};
use pathfinder_canvas::Path2D;
use geo::LineString;

pub trait ToPath {
    fn to_path(&self) -> Path2D;
}

impl ToPath for LineString<f64> {
    fn to_path(&self) -> Path2D {
        let mut path = Path2D::new();
        let mut coords = self.0.iter().copied().map(geo_coord_to_vec2f);

        if let Some(first_coord) = coords.next() {
            path.move_to(first_coord);
        }

        for coord in coords {
            path.line_to(coord);
        }

        path.close_path();

        path
    }
}

fn geo_coord_to_vec2f(geo_coord: geo::Coordinate<f64>) -> Vector2F {
    vec2f(geo_coord.x as f32, geo_coord.y as f32)
}
