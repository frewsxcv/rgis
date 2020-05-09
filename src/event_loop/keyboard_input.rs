use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};
use glutin::event_loop::ControlFlow;

use pathfinder_geometry::vector::Vector2F;

pub use super::EventLoopContext;

const PAN_FACTOR: f32 = 0.05;

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
            ctx.view_center =
                ctx.view_center - Vector2F::new(0., ctx.view_box.height() * PAN_FACTOR / ctx.scale);
            ctx.gl_context.window().request_redraw();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Down),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.view_center =
                ctx.view_center + Vector2F::new(0., ctx.view_box.height() * PAN_FACTOR / ctx.scale);
            ctx.gl_context.window().request_redraw();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Left),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.view_center =
                ctx.view_center - Vector2F::new(ctx.view_box.width() * PAN_FACTOR / ctx.scale, 0.);
            ctx.gl_context.window().request_redraw();
        }
        KeyboardInput {
            virtual_keycode: Some(VirtualKeyCode::Right),
            state: ElementState::Pressed,
            ..
        } => {
            ctx.view_center =
                ctx.view_center + Vector2F::new(ctx.view_box.width() * PAN_FACTOR / ctx.scale, 0.);
            ctx.gl_context.window().request_redraw();
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
