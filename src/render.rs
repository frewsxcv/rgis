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

pub struct RenderContext<'a> {
    pub canvas: &'a mut CanvasRenderingContext2D,
    pub extent: geo::Rect<f64>,
    pub color: ColorU,
    pub window_size: pathfinder_geometry::vector::Vector2I,
}

pub trait Render: ::std::marker::Sync + ::std::marker::Send {
    fn render(&self, ctx: RenderContext);
}

fn coords_to_screen_coords(
    coords: impl Iterator<Item = geo::Coordinate<f64>>,
    extent: geo::Rect<f64>,
    window_size: pathfinder_geometry::vector::Vector2I,
) -> impl Iterator<Item = [f64; 2]> {
    let x_scale = window_size.x() as f64 / extent.width();

    let y_scale = window_size.y() as f64 / extent.height();

    let scale = x_scale.min(y_scale);

    coords
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

impl Render for geo::LineString<f64> {
    fn render(&self, mut ctx: RenderContext) {
        render_line_string(self, &mut ctx)
    }
}

impl Render for geo::MultiLineString<f64> {
    fn render(&self, mut ctx: RenderContext) {
        for line_string in &self.0 {
            render_line_string(line_string, &mut ctx)
        }
    }
}

fn render_line_string(line_string: &geo::LineString<f64>, ctx: &mut RenderContext) {
    ctx.canvas.set_line_width(5.0);

    let mut coords = coords_to_screen_coords(line_string.0.iter().copied(), ctx.extent, ctx.window_size);
    let mut path = Path2D::new();

    if let Some(first_coord) = coords.next() {
        path.move_to(vec2f(first_coord[0] as f32, first_coord[1] as f32));
    }

    for coord in coords {
        path.line_to(vec2f(coord[0] as f32, coord[1] as f32));
    }

    path.close_path();
    ctx.canvas.set_stroke_style(ctx.color);
    ctx.canvas.stroke_path(path);
}

impl Render for geo::Polygon<f64> {
    fn render(&self, mut ctx: RenderContext) {
        render_polygon(self, &mut ctx)
    }
}

impl Render for geo::MultiPolygon<f64> {
    fn render(&self, mut ctx: RenderContext) {
        for polygon in &self.0 {
            render_polygon(polygon, &mut ctx)
        }
    }
}

fn render_polygon(polygon: &geo::Polygon<f64>, ctx: &mut RenderContext) {
    ctx.canvas.set_line_width(5.0);

    let mut coords =
        coords_to_screen_coords(polygon.exterior().0.iter().copied(), ctx.extent, ctx.window_size);
    let mut path = Path2D::new();

    if let Some(first_coord) = coords.next() {
        path.move_to(vec2f(first_coord[0] as f32, first_coord[1] as f32));
    }

    for coord in coords {
        path.line_to(vec2f(coord[0] as f32, coord[1] as f32));
    }

    path.close_path();

    ctx.canvas.set_fill_style(ctx.color);
    ctx.canvas.fill_path(path, pathfinder_content::fill::FillRule::Winding);
}

impl Render for geo::Geometry<f64> {
    fn render(&self, ctx: RenderContext) {
        match self {
            geo::Geometry::Polygon(p) => p.render(ctx),
            geo::Geometry::LineString(p) => p.render(ctx),
            geo::Geometry::MultiLineString(p) => p.render(ctx),
            geo::Geometry::MultiPolygon(p) => p.render(ctx),
            _ => (),
        }
    }
}

pub fn next_color() -> ColorU {
    *COLOR_ITER.lock().unwrap().next().unwrap()
}
