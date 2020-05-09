use glutin::dpi::PhysicalPosition;

use glutin::event::{ElementState, MouseButton};

pub use super::EventLoopContext;

pub fn handle(ctx: &mut EventLoopContext, element_state: ElementState, mouse_button: MouseButton) {
    match (mouse_button, element_state) {
        (MouseButton::Left, ElementState::Pressed) => {
            let geo_coordinate = physical_position_to_geo_coordinate(ctx, ctx.cursor_position);
            let selected_layer_changed = {
                let mut layers = ctx.layers.write().unwrap();
                layers.set_selected_layer_from_mouse_press(geo_coordinate)
            };

            log::info!(
                "Mouse clicked. Screen: (x: {}, y: {}). Geo: (x: {}, y: {}).",
                ctx.cursor_position.x,
                ctx.cursor_position.y,
                geo_coordinate.x,
                geo_coordinate.y,
            );

            if selected_layer_changed {
                ctx.build_canvas();
                ctx.gl_context.window().request_redraw();
            }
        }
        _ => {}
    }
}

fn physical_position_to_geo_coordinate(
    ctx: &EventLoopContext,
    physical_position: PhysicalPosition<f64>,
) -> geo::Coordinate<f64> {
    geo::Coordinate {
        x: ctx.view_center.x() as f64 + (physical_position.x / (ctx.scale as f64)),
        y: -(ctx.view_center.y() as f64 + (physical_position.y / (ctx.scale as f64))),
    }
}
