extern crate geo;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

mod window;

const RED: graphics::types::Color = [1., 0., 0., 1.];

fn render_polygon(geo_polygon: &geo::Polygon<f64>,
                  draw_state: &graphics::draw_state::DrawState,
                  transform: graphics::math::Matrix2d,
                  gl: &mut opengl_graphics::GlGraphics) {
    let graphics_polygon = graphics::polygon::Polygon::new(RED);

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
        use graphics::{clear, Transformed};

        let transform = ctx.transform.trans(10.0, 100.0);

        clear([0.0, 0.0, 0.0, 1.0], g);

        render_polygon(&geo_polygon, &ctx.draw_state, transform, g);
   });
}
