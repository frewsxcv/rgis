use crate::window;
use geo;
use pathfinder_canvas::{CanvasRenderingContext2D, ColorU, Path2D};
use pathfinder_geometry::vector::vec2f;
use std::iter;
use std::slice;
use std::sync;

static COLORS: [ColorU; 7] = [
    // Red
    ColorU {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    },
    // Orange
    ColorU {
        r: 255,
        g: 127,
        b: 0,
        a: 255,
    },
    // Yellow
    ColorU {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    },
    // Green
    ColorU {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    },
    // Blue
    ColorU {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    },
    // Indigo
    ColorU {
        r: 75,
        g: 0,
        b: 130,
        a: 255,
    },
    // Violet
    ColorU {
        r: 148,
        g: 0,
        b: 211,
        a: 255,
    },
];

lazy_static::lazy_static! {
    static ref COLOR_ITER: sync::Mutex<iter::Cycle<slice::Iter<'static, ColorU>>> = {
        sync::Mutex::new(COLORS.iter().cycle())
    };
}

pub trait Renderable: ::std::marker::Sync + ::std::marker::Send {
    fn render(&self, canvas: &mut CanvasRenderingContext2D, extent: geo::Rect<f64>);
}

fn line_string_to_screen_coords<'a>(
    line_string: &'a geo::LineString<f64>,
    extent: geo::Rect<f64>,
) -> impl Iterator<Item = [f64; 2]> + 'a {
    let x_scale = window::WINDOW_SIZE_X as f64 / extent.width();

    let y_scale = window::WINDOW_SIZE_Y as f64 / extent.height();

    let scale = x_scale.min(y_scale);

    line_string
        .0
        .iter()
        .map(move |coord| geo::Coordinate {
            x: coord.x - extent.min().x,
            y: coord.y - extent.max().y,
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
    fn render(&self, canvas: &mut CanvasRenderingContext2D, extent: geo::Rect<f64>) {
        render_line_string(self, canvas, extent, next_color())
    }
}

impl Renderable for geo::MultiLineString<f64> {
    fn render(&self, canvas: &mut CanvasRenderingContext2D, extent: geo::Rect<f64>) {
        let color = next_color();
        for line_string in &self.0 {
            render_line_string(line_string, canvas, extent, color)
        }
    }
}

fn render_line_string(
    polygon: &geo::LineString<f64>,
    canvas: &mut CanvasRenderingContext2D,
    extent: geo::Rect<f64>,
    color: ColorU,
) {
    canvas.set_line_width(5.0);

    let mut coords = line_string_to_screen_coords(polygon, extent);
    let mut path = Path2D::new();

    if let Some(first_coord) = coords.next() {
        path.move_to(vec2f(first_coord[0] as f32, first_coord[1] as f32));
    }

    for coord in coords {
        path.line_to(vec2f(coord[0] as f32, coord[1] as f32));
    }

    path.close_path();
    canvas.set_stroke_style(color);
    canvas.stroke_path(path);
}

impl Renderable for geo::Polygon<f64> {
    fn render(&self, canvas: &mut CanvasRenderingContext2D, extent: geo::Rect<f64>) {
        render_polygon(self, canvas, extent, next_color())
    }
}

impl Renderable for geo::MultiPolygon<f64> {
    fn render(&self, canvas: &mut CanvasRenderingContext2D, extent: geo::Rect<f64>) {
        let color = next_color();
        for polygon in &self.0 {
            render_polygon(polygon, canvas, extent, color)
        }
    }
}

fn render_polygon(
    polygon: &geo::Polygon<f64>,
    canvas: &mut CanvasRenderingContext2D,
    extent: geo::Rect<f64>,
    color: ColorU,
) {
    canvas.set_line_width(5.0);

    let mut coords = line_string_to_screen_coords(polygon.exterior(), extent);
    let mut path = Path2D::new();

    if let Some(first_coord) = coords.next() {
        path.move_to(vec2f(first_coord[0] as f32, first_coord[1] as f32));
    }

    for coord in coords {
        path.line_to(vec2f(coord[0] as f32, coord[1] as f32));
    }

    path.close_path();

    canvas.set_fill_style(color);
    canvas.fill_path(path, pathfinder_content::fill::FillRule::Winding);
}

impl Renderable for geo::Geometry<f64> {
    fn render(&self, canvas: &mut CanvasRenderingContext2D, extent: geo::Rect<f64>) {
        match self {
            geo::Geometry::Polygon(p) => p.render(canvas, extent),
            geo::Geometry::LineString(p) => p.render(canvas, extent),
            geo::Geometry::MultiLineString(p) => p.render(canvas, extent),
            geo::Geometry::MultiPolygon(p) => p.render(canvas, extent),
            _ => (),
        }
    }
}

fn next_color() -> ColorU {
    *COLOR_ITER.lock().unwrap().next().unwrap()
}
