use graphics;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window;

pub const WINDOW_SIZE_X: u32 = 800;
pub const WINDOW_SIZE_Y: u32 = 800;
pub const WINDOW_SIZE: [u32; 2] = [WINDOW_SIZE_X, WINDOW_SIZE_Y];

const OPEN_GL_VERSION: OpenGL = OpenGL::V3_2;

fn build_window() -> Sdl2Window {
    WindowSettings::new(crate::PROGRAM_NAME, WINDOW_SIZE)
        .exit_on_esc(true)
        .opengl(OPEN_GL_VERSION)
        .build()
        .unwrap()
}

pub fn window_loop<F>(f: F)
where
    F: Fn(graphics::Context, &mut GlGraphics),
{
    let mut window = build_window();
    let mut gl = GlGraphics::new(OPEN_GL_VERSION);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), &f);
        }
    }
}
