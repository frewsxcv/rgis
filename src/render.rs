use geo;
use pathfinder_canvas::{CanvasRenderingContext2D, ColorU};
use crate::canvas::to_path::ToPath;

pub struct RenderContext<'a> {
    pub canvas: &'a mut CanvasRenderingContext2D,
    pub color: ColorU,
    pub scale: f32,
    pub selected: bool,
}

pub trait Render: ::std::marker::Sync + ::std::marker::Send {
    fn render(&self, ctx: RenderContext);
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
    let path = line_string.to_path();
    let color = if ctx.selected {
        ColorU::black()
    } else {
        ctx.color
    };
    ctx.canvas.set_line_width(5.0 / ctx.scale);
    ctx.canvas.set_stroke_style(color);
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
    let path = polygon.exterior().to_path();
    let color = if ctx.selected {
        ColorU::black()
    } else {
        ctx.color
    };
    ctx.canvas.set_line_width(5.0 / ctx.scale);
    ctx.canvas.set_fill_style(color);
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
