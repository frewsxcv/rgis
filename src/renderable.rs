use crate::window;
use geo;
use geo::bounding_rect::BoundingRect;
use pathfinder_canvas::{CanvasRenderingContext2D, ColorU, Path2D};
use pathfinder_geometry::vector::vec2f;
use std::iter;
use std::slice;
use std::sync;

static COLORS: [ColorU; 3] = [
    ColorU {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    },
    ColorU {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    },
    ColorU {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    },
];

lazy_static::lazy_static! {
    static ref COLOR_ITER: sync::Mutex<iter::Cycle<slice::Iter<'static, ColorU>>> = {
        sync::Mutex::new(COLORS.iter().cycle())
    };
}

pub trait Renderable: ::std::marker::Sync + ::std::marker::Send {
    fn render(&self, canvas: &mut CanvasRenderingContext2D);
}

fn line_string_to_screen_coords<'a>(
    line_string: &'a geo::LineString<f64>,
) -> impl Iterator<Item = [f64; 2]> + 'a {
    let bbox = line_string.bounding_rect().unwrap();

    let x_scale = window::WINDOW_SIZE_X as f64 / bbox.width();

    let y_scale = window::WINDOW_SIZE_Y as f64 / bbox.height();

    let scale = x_scale.min(y_scale);

    line_string
        .0
        .iter()
        .map(move |coord| geo::Coordinate {
            x: coord.x - bbox.min().x,
            y: coord.y - bbox.max().y,
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
    fn render(&self, canvas: &mut CanvasRenderingContext2D) {
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
        canvas.set_stroke_style(next_color());
        canvas.stroke_path(path);
    }
}

impl Renderable for geo::Polygon<f64> {
    fn render(&self, canvas: &mut CanvasRenderingContext2D) {
        canvas.set_line_width(5.0);

        let mut coords = line_string_to_screen_coords(self.exterior());
        let mut path = Path2D::new();

        if let Some(first_coord) = coords.next() {
            path.move_to(vec2f(first_coord[0] as f32, first_coord[1] as f32));
        }

        for coord in coords {
            path.line_to(vec2f(coord[0] as f32, coord[1] as f32));
        }

        path.close_path();

        canvas.set_fill_style(next_color());
        canvas.fill_path(path, pathfinder_content::fill::FillRule::Winding);
    }
}

fn next_color() -> ColorU {
    *COLOR_ITER.lock().unwrap().next().unwrap()
}
