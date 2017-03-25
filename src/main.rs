extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

use piston::event_loop::{Events, EventSettings};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use opengl_graphics::{OpenGL, GlGraphics};
use sdl2_window::Sdl2Window;
use graphics::{clear, polygon, Transformed};

const RED: graphics::types::Color = [1., 0., 0., 1.];

const WINDOW_TITLE: &'static str = "rgis";
const WINDOW_SIZE: [u32; 2] = [800u32, 800u32];

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Sdl2Window = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |ctx, g| {
                let transform = ctx.transform.trans(10.0, 100.0);

                clear([0.0, 0.0, 0.0, 1.0], g);

                let polygon = polygon::Polygon::new(RED);

                let square = [[0., 0.], [10., 0.], [10., 10.], [0., 10.]];

                polygon.draw(&square, &ctx.draw_state, transform, g);
            });
        }
    }
}
