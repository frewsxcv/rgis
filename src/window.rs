use graphics;
use piston::event_loop::{Events, EventSettings};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use opengl_graphics::{OpenGL, GlGraphics};
use sdl2_window::Sdl2Window;

const WINDOW_TITLE: &'static str = "rgis";
pub const WINDOW_SIZE_X: u32 = 800;
pub const WINDOW_SIZE_Y: u32 = 800;
pub const WINDOW_SIZE: [u32; 2] = [WINDOW_SIZE_X, WINDOW_SIZE_Y];

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
