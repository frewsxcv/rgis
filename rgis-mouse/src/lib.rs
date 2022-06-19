#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::ecs::system::{Query, Res, ResMut};

#[derive(Copy, Clone)]
pub struct MousePos {
    pub projected: geo::Coordinate<f64>,
}

fn cursor_moved_system(
    mut cursor_moved_event_reader: bevy::ecs::event::EventReader<bevy::window::CursorMoved>,
    windows: Res<bevy::window::Windows>,
    camera_2d: Res<rgis_camera::Camera2d>,
    query: Query<&mut bevy::transform::components::Transform>,
    mut mouse_position: ResMut<MousePos>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
) {
    if cursor_moved_event_reader.is_empty() {
        return;
    }
    if bevy_egui_ctx.ctx_mut().is_pointer_over_area() {
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
) -> geo::Coordinate<f64> {
    let size = bevy::math::Vec2::new(window.width() as f32, window.height() as f32);

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let p = screen_coords - size / 2.0;

    // apply the camera transform
    let pos_wld = transform.compute_matrix() * p.extend(0.0).extend(1.0);

    geo::Coordinate {
        x: pos_wld.x.into(),
        y: pos_wld.y.into(),
    }
}

fn mouse_motion_system(
    mut mouse_motion_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseMotion>,
    mut map_clicked_event_writer: bevy::ecs::event::EventWriter<rgis_events::MapClickedEvent>,
    mouse_button: Res<bevy::input::Input<bevy::input::mouse::MouseButton>>,
    mut pan_camera_events: bevy::ecs::event::EventWriter<rgis_events::PanCameraEvent>,
    mut windows: ResMut<bevy::window::Windows>,
    mut bevy_egui_ctx: ResMut<bevy_egui::EguiContext>,
    rgis_settings: Res<rgis_settings::RgisSettings>,
    mouse_position: Res<MousePos>,
) {
    if bevy_egui_ctx.ctx_mut().is_pointer_over_area() {
        windows
            .primary_mut()
            .set_cursor_icon(bevy::window::CursorIcon::Arrow);
        return;
    }

    match rgis_settings.current_tool {
        rgis_settings::Tool::Pan => {
            if mouse_button.pressed(bevy::input::mouse::MouseButton::Left)
                || mouse_button.pressed(bevy::input::mouse::MouseButton::Right)
            {
                windows
                    .primary_mut()
                    .set_cursor_icon(bevy::window::CursorIcon::Grabbing);
                let mut x_sum = 0.;
                let mut y_sum = 0.;
                for event in mouse_motion_event_reader.iter() {
                    // If the mouse is dragging rightward, `delta.x` will be positive. In this case, we
                    // want the map to move right, and the camera to move left. We need to negate the
                    // delta X value.
                    x_sum -= event.delta.x;

                    // If the mouse is dragging upward, `delta.y` will be negative. In this case, we
                    // want the map to move up, and the camera to move down. We do not need to negate
                    // the delta Y value.
                    y_sum += event.delta.y;
                }
                if x_sum != 0. && y_sum != 0. {
                    pan_camera_events.send(rgis_events::PanCameraEvent { x: x_sum, y: y_sum });
                }
                return;
            }
        }
        rgis_settings::Tool::Query => {
            if mouse_button.just_pressed(bevy::input::mouse::MouseButton::Left) {
                map_clicked_event_writer
                    .send(rgis_events::MapClickedEvent(mouse_position.projected))
            }
        }
    }

    let cursor_icon = match rgis_settings.current_tool {
        rgis_settings::Tool::Pan => bevy::window::CursorIcon::Grab,
        rgis_settings::Tool::Query => bevy::window::CursorIcon::Crosshair,
    };
    windows.primary_mut().set_cursor_icon(cursor_icon);
}

fn mouse_scroll_system(
    mut mouse_scroll_event_reader: bevy::ecs::event::EventReader<bevy::input::mouse::MouseWheel>,
    mut zoom_camera_events: bevy::ecs::event::EventWriter<rgis_events::ZoomCameraEvent>,
) {
    let mut x = 0.;
    for event in mouse_scroll_event_reader.iter() {
        x += event.y;
    }
    if x != 0. {
        zoom_camera_events.send(rgis_events::ZoomCameraEvent::new(x));
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system(cursor_moved_system)
            .add_system(mouse_scroll_system)
            .add_system(mouse_motion_system)
            .insert_resource(MousePos {
                projected: geo::Coordinate { x: 0., y: 0. },
            });
    }
}
