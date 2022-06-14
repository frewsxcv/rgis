#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::ecs::system::{Query, Res, ResMut};

#[derive(Copy, Clone)]
pub struct Xy {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct MousePos {
    pub projected: Xy,
}

fn cursor_moved_system(
    mut cursor_moved_event_reader: bevy::ecs::event::EventReader<bevy::window::CursorMoved>,
    windows: Res<bevy::window::Windows>,
    camera_2d: Res<rgis_camera::Camera2d>,
    query: Query<&mut bevy::transform::components::Transform>,
    mut mouse_position: ResMut<MousePos>,
) {
    if cursor_moved_event_reader.is_empty() {
        return;
    }
    let window = windows.primary();
    let transform = match query.get(camera_2d.0) {
        Ok(transform) => transform,
        Err(_) => return,
    };
    if let Some(event) = cursor_moved_event_reader.iter().next_back() {
        mouse_position.projected = screen_coords_to_geo_coords(event.position, transform, window);
    }
}

fn screen_coords_to_geo_coords(
    screen_coords: bevy::prelude::Vec2,
    transform: &bevy::transform::components::Transform,
    window: &bevy::prelude::Window,
) -> Xy {
    let size = bevy::math::Vec2::new(window.width() as f32, window.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let p = screen_coords - size / 2.0;

    // apply the camera transform
    let pos_wld = transform.compute_matrix() * p.extend(0.0).extend(1.0);

    Xy {
        x: pos_wld.x,
        y: pos_wld.y,
    }
}

fn mouse_motion_system(
    mut mouse_motion_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseMotion>,
    mouse_button: Res<bevy::input::Input<bevy::input::mouse::MouseButton>>,
    mut pan_camera_events: bevy::ecs::event::EventWriter<rgis_events::PanCameraEvent>,
    mut windows: ResMut<bevy::window::Windows>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
) {
    if bevy_egui_ctx.ctx_mut().is_pointer_over_area() {
        windows
            .primary_mut()
            .set_cursor_icon(bevy::window::CursorIcon::Arrow);
    } else if mouse_button.pressed(bevy::input::mouse::MouseButton::Left)
        || mouse_button.pressed(bevy::input::mouse::MouseButton::Right)
    {
        windows
            .primary_mut()
            .set_cursor_icon(bevy::window::CursorIcon::Grabbing);
        for event in mouse_motion_event_reader.iter() {
            // sum up x + y values and send one event
            pan_camera_events.send(rgis_events::PanCameraEvent {
                // If the mouse is dragging rightward, `delta.x` will be positive. In this case, we
                // want the map to move right, and the camera to move left. We need to negate the
                // delta X value.
                x: -event.delta.x,
                // If the mouse is dragging upward, `delta.y` will be negative. In this case, we
                // want the map to move up, and the camera to move down. We do not need to negate
                // the delta Y value.
                y: event.delta.y,
            });
        }
    } else {
        windows
            .primary_mut()
            .set_cursor_icon(bevy::window::CursorIcon::Grab);
    }
}

fn mouse_scroll_system(
    mut mouse_scroll_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseWheel>,
    mut zoom_camera_events: bevy::ecs::event::EventWriter<rgis_events::ZoomCameraEvent>,
) {
    for event in mouse_scroll_event_reader.iter() {
        if event.y > 0. {
            zoom_camera_events.send(rgis_events::ZoomCameraEvent::zoom_in());
        } else if event.y < 0. {
            zoom_camera_events.send(rgis_events::ZoomCameraEvent::zoom_out());
        }
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system(cursor_moved_system)
            .add_system(mouse_scroll_system)
            .add_system(mouse_motion_system)
            .insert_resource(MousePos {
                projected: Xy { x: 0., y: 0. },
            });
    }
}
