use geo;
use pathfinder_canvas::{CanvasRenderingContext2D, ColorU, Path2D};
use pathfinder_geometry::vector::vec2f;

pub struct RenderContext<'a> {
    pub canvas: &'a mut CanvasRenderingContext2D,
    pub extent: geo::Rect<f64>,
    pub color: ColorU,
    pub scale: f32,
}

pub trait Render: ::std::marker::Sync + ::std::marker::Send {
    fn render(&self, ctx: RenderContext);
}

fn coords_to_screen_coords(
    coords: impl Iterator<Item = geo::Coordinate<f64>>,
) -> impl Iterator<Item = [f64; 2]> {
    coords
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
    let path = line_string_to_path(line_string);
    ctx.canvas.set_line_width(5.0 / ctx.scale);
    ctx.canvas.set_stroke_style(ctx.color);
    ctx.canvas.stroke_path(path);
}

fn line_string_to_path(line_string: &geo::LineString<f64>) -> Path2D {
    let mut path = Path2D::new();
    let mut coords = coords_to_screen_coords(line_string.0.iter().copied());

    if let Some(first_coord) = coords.next() {
        path.move_to(vec2f(first_coord[0] as f32, first_coord[1] as f32));
    }

    for coord in coords {
        path.line_to(vec2f(coord[0] as f32, coord[1] as f32));
    }

    path.close_path();

    path
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
    let path = line_string_to_path(polygon.exterior());
    ctx.canvas.set_line_width(5.0 / ctx.scale);
    ctx.canvas.set_fill_style(ctx.color);
    ctx.canvas
        .fill_path(path, pathfinder_content::fill::FillRule::Winding);
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
