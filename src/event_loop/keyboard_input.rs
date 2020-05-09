use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};
use glutin::event_loop::ControlFlow;

pub use super::EventLoopContext;

pub fn handle(
    ctx: &mut EventLoopContext,
    keyboard_input: KeyboardInput,
    control_flow: &mut ControlFlow,
) {
    match keyboard_input {
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Escape),
            ..
        } => *control_flow = ControlFlow::Exit,
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Up),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.pan_up();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Down),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.pan_down();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Left),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.pan_left();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Right),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.pan_right();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Equals),
            state: ElementState::Pressed,
            ..
        } => {
            if ctx.shift_pressed {
                ctx.zoom_in();
            }
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Minus),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.zoom_out();
        }
        _ => {
            *control_flow = ControlFlow::Wait;
        }
    }
}
