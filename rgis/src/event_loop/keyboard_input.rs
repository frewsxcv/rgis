use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};
use glutin::event_loop::ControlFlow;

pub use super::EventLoopContext;

pub fn handle(
    ctx: &mut EventLoopContext,
    keyboard_input: KeyboardInput,
    control_flow: &mut ControlFlow,
) {
    let virtual_keycode = match keyboard_input.virtual_keycode {
        Some(k) => k,
        None => return,
    };

    match (virtual_keycode, keyboard_input.state) {
        (VirtualKeyCode::Up, ElementState::Pressed) => ctx.pan_up(),
        (VirtualKeyCode::Down, ElementState::Pressed) => ctx.pan_down(),
        (VirtualKeyCode::Left, ElementState::Pressed) => ctx.pan_left(),
        (VirtualKeyCode::Right, ElementState::Pressed) => ctx.pan_right(),
        (VirtualKeyCode::Equals, ElementState::Pressed) if ctx.shift_pressed => ctx.zoom_in(),
        (VirtualKeyCode::Minus, ElementState::Pressed) => ctx.zoom_out(),
        _ => *control_flow = ControlFlow::Wait,
    }
}
