extern crate geo;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

use geo::boundingbox::BoundingBox;
use graphics::{clear, Transformed};

mod window;

const RED: graphics::types::Color = [1., 0., 0., 1.];

fn render_polygon(geo_polygon: &geo::Polygon<f64>,
                  draw_state: &graphics::draw_state::DrawState,
                  transform: graphics::math::Matrix2d,
                  gl: &mut opengl_graphics::GlGraphics) {
    let graphics_polygon = graphics::polygon::Polygon::new(RED);

    let bbox = geo_polygon.bbox().unwrap();

    let bbox_width = bbox.xmax - bbox.xmin;
    let x_scale = window::WINDOW_SIZE_X as f64 / bbox_width;

    let bbox_height = bbox.ymax - bbox.ymin;
    let y_scale = window::WINDOW_SIZE_Y as f64 / bbox_height;

    let transform = transform.scale(x_scale, y_scale);

    let points = geo_polygon.exterior.0
        .iter()
        .map(|point| point.0)
        .map(|coord| [coord.x, coord.y])
        .collect::<Vec<_>>();

    graphics_polygon.draw(&points, draw_state, transform, gl);
}

fn main() {
    let geo_polygon = geo::Polygon {
        exterior: geo::LineString(
            vec![
                geo::Point(geo::Coordinate { x: 0., y: 0. }),
                geo::Point(geo::Coordinate { x: 10., y: 0. }),
                geo::Point(geo::Coordinate { x: 10., y: 10. }),
                geo::Point(geo::Coordinate { x: 0., y: 10. }),
            ]
        ),
        interiors: vec![],
    };

    window::window_loop(|ctx, g| {
        clear([0.0, 0.0, 0.0, 1.0], g);

        render_polygon(&geo_polygon, &ctx.draw_state, ctx.transform, g);
   });
}
