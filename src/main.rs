// extern crate geo;
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

mod window;

fn main() {
    window::window_loop(|ctx, g| {
        use graphics::{clear, Transformed};

        let transform = ctx.transform.trans(10.0, 100.0);

        clear([0.0, 0.0, 0.0, 1.0], g);

        window::render_polygon(&ctx.draw_state, transform, g);
   });
}
