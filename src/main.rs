// extern crate geo;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

mod window;

const RED: graphics::types::Color = [1., 0., 0., 1.];

fn render_polygon(draw_state: &graphics::draw_state::DrawState,
                  transform: graphics::math::Matrix2d,
                  gl: &mut opengl_graphics::GlGraphics) {
    let polygon = graphics::polygon::Polygon::new(RED);

    let square = [[0., 0.], [10., 0.], [10., 10.], [0., 10.]];

    polygon.draw(&square, draw_state, transform, gl);
}

fn main() {
    window::window_loop(|ctx, g| {
        use graphics::{clear, Transformed};

        let transform = ctx.transform.trans(10.0, 100.0);

        clear([0.0, 0.0, 0.0, 1.0], g);

        render_polygon(&ctx.draw_state, transform, g);
   });
}
