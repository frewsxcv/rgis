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

pub trait Render: ::std::marker::Sync + ::std::marker::Send {
    fn render(
        &self,
        canvas: &mut CanvasRenderingContext2D,
        extent: geo::Rect<f64>,
        color: ColorU,
        window_size: pathfinder_geometry::vector::Vector2I,
    );
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
    fn render(
        &self,
        canvas: &mut CanvasRenderingContext2D,
        extent: geo::Rect<f64>,
        color: ColorU,
        window_size: pathfinder_geometry::vector::Vector2I,
    ) {
        render_line_string(self, canvas, extent, color, window_size)
    }
}

impl Render for geo::MultiLineString<f64> {
    fn render(
        &self,
        canvas: &mut CanvasRenderingContext2D,
        extent: geo::Rect<f64>,
        color: ColorU,
        window_size: pathfinder_geometry::vector::Vector2I,
    ) {
        for line_string in &self.0 {
            render_line_string(line_string, canvas, extent, color, window_size)
        }
    }
}

fn render_line_string(
    line_string: &geo::LineString<f64>,
    canvas: &mut CanvasRenderingContext2D,
    extent: geo::Rect<f64>,
    color: ColorU,
    window_size: pathfinder_geometry::vector::Vector2I,
) {
    canvas.set_line_width(5.0);

    let mut coords = coords_to_screen_coords(line_string.0.iter().copied(), extent, window_size);
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

impl Render for geo::Polygon<f64> {
    fn render(
        &self,
        canvas: &mut CanvasRenderingContext2D,
        extent: geo::Rect<f64>,
        color: ColorU,
        window_size: pathfinder_geometry::vector::Vector2I,
    ) {
        render_polygon(self, canvas, extent, color, window_size)
    }
}

impl Render for geo::MultiPolygon<f64> {
    fn render(
        &self,
        canvas: &mut CanvasRenderingContext2D,
        extent: geo::Rect<f64>,
        color: ColorU,
        window_size: pathfinder_geometry::vector::Vector2I,
    ) {
        for polygon in &self.0 {
            render_polygon(polygon, canvas, extent, color, window_size)
        }
    }
}

fn render_polygon(
    polygon: &geo::Polygon<f64>,
    canvas: &mut CanvasRenderingContext2D,
    extent: geo::Rect<f64>,
    color: ColorU,
    window_size: pathfinder_geometry::vector::Vector2I,
) {
    canvas.set_line_width(5.0);

    let mut coords =
        coords_to_screen_coords(polygon.exterior().0.iter().copied(), extent, window_size);
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

impl Render for geo::Geometry<f64> {
    fn render(
        &self,
        canvas: &mut CanvasRenderingContext2D,
        extent: geo::Rect<f64>,
        color: ColorU,
        window_size: pathfinder_geometry::vector::Vector2I,
    ) {
        match self {
            geo::Geometry::Polygon(p) => p.render(canvas, extent, color, window_size),
            geo::Geometry::LineString(p) => p.render(canvas, extent, color, window_size),
            geo::Geometry::MultiLineString(p) => p.render(canvas, extent, color, window_size),
            geo::Geometry::MultiPolygon(p) => p.render(canvas, extent, color, window_size),
            _ => (),
        }
    }
}

pub fn next_color() -> ColorU {
    *COLOR_ITER.lock().unwrap().next().unwrap()
}
