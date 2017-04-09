use geo;
use geo::boundingbox::BoundingBox;
use opengl_graphics;
use graphics::{self, Transformed};
use window;

pub trait Renderable: ::std::marker::Sync + ::std::marker::Send {
    fn render(&self,
              draw_state: graphics::draw_state::DrawState,
              transform: graphics::math::Matrix2d,
              gl: &mut opengl_graphics::GlGraphics);
}

impl Renderable for geo::LineString<f64> {
    fn render(&self,
              draw_state: graphics::draw_state::DrawState,
              transform: graphics::math::Matrix2d,
              gl: &mut opengl_graphics::GlGraphics) {
        let graphics_line = graphics::line::Line::new(::RED, 1.);

        let bbox = self.bbox().unwrap();

        let bbox_width = bbox.xmax - bbox.xmin;
        let x_scale = window::WINDOW_SIZE_X as f64 / bbox_width;

        let bbox_height = bbox.ymax - bbox.ymin;
        let y_scale = window::WINDOW_SIZE_Y as f64 / bbox_height;

        let scale = x_scale.min(y_scale);

        //let transform = transform.trans(-bbox.xmin, -bbox.ymin);
        let transform = transform.flip_v();
        //let transform = transform.scale(x_scale, y_scale);

        let points = self.0
            .iter()
            .map(|point| point.0)
            .map(|coord| {
                     geo::Coordinate {
                         x: coord.x - bbox.xmin,
                         y: coord.y - bbox.ymax,
                     }
                 })
            .map(|coord| {
                     geo::Coordinate {
                         x: coord.x * scale,
                         y: coord.y * scale,
                     }
                 })
            .map(|coord| [coord.x, coord.y])
            .collect::<Vec<_>>();

        for x in points.windows(2) {
            graphics_line.draw([x[0][0], x[0][1], x[1][0], x[1][1]],
                               &draw_state,
                               transform,
                               gl);
        }
    }
}

impl Renderable for geo::Polygon<f64> {
    fn render(&self,
              draw_state: graphics::draw_state::DrawState,
              transform: graphics::math::Matrix2d,
              gl: &mut opengl_graphics::GlGraphics) {
        let graphics_polygon = graphics::polygon::Polygon::new(::RED);

        let bbox = self.bbox().unwrap();

        let bbox_width = bbox.xmax - bbox.xmin;
        let x_scale = window::WINDOW_SIZE_X as f64 / bbox_width;

        let bbox_height = bbox.ymax - bbox.ymin;
        let y_scale = window::WINDOW_SIZE_Y as f64 / bbox_height;

        let scale = x_scale.min(y_scale);

        //let transform = transform.trans(-bbox.xmin, -bbox.ymin);
        let transform = transform.flip_v();
        //let transform = transform.scale(x_scale, y_scale);

        let points = self.exterior
            .0
            .iter()
            .map(|point| point.0)
            .map(|coord| {
                     geo::Coordinate {
                         x: coord.x - bbox.xmin,
                         y: coord.y - bbox.ymax,
                     }
                 })
            .map(|coord| {
                     geo::Coordinate {
                         x: coord.x * scale,
                         y: coord.y * scale,
                     }
                 })
            .map(|coord| [coord.x, coord.y]);

        let mut polygon = ::poly2tri::Polygon::new();
        for p in points.skip(1) {
            polygon.add_point(p[0], p[1]);
        }
        let cdt = ::poly2tri::CDT::new(polygon);
        let triangle_vec = cdt.triangulate();

        for i in 0..triangle_vec.size() {
            let triangle = triangle_vec.get_triangle(i);
            graphics_polygon.draw(&triangle.points, &draw_state, transform, gl);
        }
    }
}
