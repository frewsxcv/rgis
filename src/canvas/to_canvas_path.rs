use geo::LineString;

pub trait ToCanvasPath {
    fn to_canvas_path(&self) -> rgx::kit::shape2d::Line;
}

impl ToCanvasPath for LineString<f64> {
    fn to_canvas_path(&self) -> rgx::kit::shape2d::Line {
        let mut coords = line_string_to_canvas_coords(self);

        let mut path = Path2D::new();

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

fn line_string_to_canvas_coords<'a>(
    line_string: &'a geo::LineString<f64>,
) -> impl Iterator<Item = Vector2F> + 'a {
    line_string.0.iter().copied().map(geo_coord_to_vec2f)
}

fn geo_coord_to_vec2f(geo_coord: geo::Coordinate<f64>) -> Vector2F {
    vec2f(geo_coord.x as f32, geo_coord.y as f32)
}
