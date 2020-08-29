use super::to_canvas_path::ToCanvasPath;
use geo;
use rgx::kit::shape2d::Shape;

pub trait AddToCanvas {
    fn add_to_canvas(&self, ctx: Context);
}

pub struct Context {
    pub shape_batch: rgx::kit::shape2d::Batch,
    pub color: rgx::color::Rgba8,
    pub scale: f32,
    pub selected: bool,
}

impl AddToCanvas for geo::LineString<f64> {
    fn add_to_canvas(&self, mut ctx: Context) {
        render_line_string(self, &mut ctx)
    }
}

impl AddToCanvas for geo::MultiLineString<f64> {
    fn add_to_canvas(&self, mut ctx: Context) {
        for line_string in &self.0 {
            render_line_string(line_string, &mut ctx)
        }
    }
}

fn render_line_string(line_string: &geo::LineString<f64>, ctx: &mut Context) {
    let path = line_string.to_canvas_path();
    let color = if ctx.selected {
        rgx::color::Rgba8::BLACK
    } else {
        ctx.color
    };
    ctx.canvas.set_line_width(5.0 / ctx.scale);
    ctx.canvas.set_stroke_style(color);
    ctx.canvas.stroke_path(path);
}

impl AddToCanvas for geo::Polygon<f64> {
    fn add_to_canvas(&self, mut ctx: Context) {
        render_polygon(self, &mut ctx)
    }
}

impl AddToCanvas for geo::MultiPolygon<f64> {
    fn add_to_canvas(&self, mut ctx: Context) {
        for polygon in &self.0 {
            render_polygon(polygon, &mut ctx)
        }
    }
}

fn render_polygon(polygon: &geo::Polygon<f64>, ctx: &mut Context) {
    let path = polygon.exterior().to_canvas_path();
    let color = if ctx.selected {
        rgx::color::Rgba8::BLACK
    } else {
        ctx.color
    };
    ctx.canvas.set_line_width(5.0 / ctx.scale);
    ctx.canvas.set_fill_style(color);
    ctx.canvas
        .fill_path(path, pathfinder_content::fill::FillRule::Winding);
}

impl AddToCanvas for geo::Geometry<f64> {
    fn add_to_canvas(&self, ctx: Context) {
        match self {
            geo::Geometry::Polygon(p) => p.add_to_canvas(ctx),
            geo::Geometry::LineString(p) => p.add_to_canvas(ctx),
            geo::Geometry::MultiLineString(p) => p.add_to_canvas(ctx),
            geo::Geometry::MultiPolygon(p) => p.add_to_canvas(ctx),
            _ => (),
        }
    }
}
