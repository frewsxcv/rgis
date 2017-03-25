use graphics;
use piston::event_loop::{Events, EventSettings};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use opengl_graphics::{OpenGL, GlGraphics};
use sdl2_window::Sdl2Window;

const RED: graphics::types::Color = [1., 0., 0., 1.];

const WINDOW_TITLE: &'static str = "rgis";
const WINDOW_SIZE: [u32; 2] = [800u32, 800u32];

pub fn render_polygon(draw_state: &graphics::draw_state::DrawState,
                  transform: graphics::math::Matrix2d,
                  gl: &mut GlGraphics) {
    let polygon = graphics::polygon::Polygon::new(RED);

    let square = [[0., 0.], [10., 0.], [10., 10.], [0., 10.]];

    polygon.draw(&square, draw_state, transform, gl);
}

pub fn window_loop<F>(f: F)
    where F: Fn(graphics::Context, &mut GlGraphics)
{
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
            gl.draw(args.viewport(), &f);
        }
    }
}
