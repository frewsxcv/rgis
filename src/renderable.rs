use geo;
use geo::boundingbox::BoundingBox;
use crate::window;
use pathfinder_canvas::{Path2D, CanvasRenderingContext2D};
use pathfinder_geometry::vector::{vec2f};

pub trait Renderable: ::std::marker::Sync + ::std::marker::Send {
    fn render(
        &self,
        canvas: &mut CanvasRenderingContext2D,
    );
}

fn line_string_to_screen_coords<'a>(
    line_string: &'a geo::LineString<f64>,
) -> impl Iterator<Item = [f64; 2]> + 'a {
    let bbox = line_string.bbox().unwrap();

    let bbox_width = bbox.xmax - bbox.xmin;
    let x_scale = window::WINDOW_SIZE_X as f64 / bbox_width;

    let bbox_height = bbox.ymax - bbox.ymin;
    let y_scale = window::WINDOW_SIZE_Y as f64 / bbox_height;

    let scale = x_scale.min(y_scale);

    line_string
        .0
        .iter()
        .map(|point| point.0)
        .map(move |coord| geo::Coordinate {
            x: coord.x - bbox.xmin,
            y: coord.y - bbox.ymax,
        })
        .map(move |coord| geo::Coordinate {
            x: coord.x * scale,
            y: coord.y * scale,
        })
        .map(move |coord| geo::Coordinate {
            x: coord.x,
            y: -coord.y,
        })
        .map(|coord| [coord.x, coord.y])
}

impl Renderable for geo::LineString<f64> {
    fn render(
        &self,
        canvas: &mut CanvasRenderingContext2D,
    ) {
        canvas.set_line_width(5.0);

        let mut coords = line_string_to_screen_coords(self);
        let mut path = Path2D::new();

        if let Some(first_coord) = coords.next() {
            path.move_to(vec2f(first_coord[0] as f32, first_coord[1] as f32));
        }

        for coord in coords {
            path.line_to(vec2f(coord[0] as f32, coord[1] as f32));
        }

        path.close_path();
        canvas.stroke_path(path);
    }
}

impl Renderable for geo::Polygon<f64> {
    fn render(
        &self,
        canvas: &mut CanvasRenderingContext2D,
    ) {
        canvas.set_line_width(5.0);

        let mut coords = line_string_to_screen_coords(&self.exterior);
        let mut path = Path2D::new();

        if let Some(first_coord) = coords.next() {
            path.move_to(vec2f(first_coord[0] as f32, first_coord[1] as f32));
        }

        for coord in coords {
            path.line_to(vec2f(coord[0] as f32, coord[1] as f32));
        }

        path.close_path();
        canvas.fill_path(path, pathfinder_content::fill::FillRule::Winding);
    }
}
