use bevy::ecs::system::{IntoSystem, Query, Res, ResMut};

fn cursor_moved_system(
    mut cursor_moved_event_reader: bevy::app::EventReader<bevy::window::CursorMoved>,
    windows: Res<bevy::window::Windows>,
    camera_transform_query: Query<(
        &rgis_camera::Camera2d,
        &bevy::transform::components::Transform,
    )>,
    mut ui_state: ResMut<rgis_ui::UiState>,
) {
    for event in cursor_moved_event_reader.iter() {
        for (_camera, transform) in camera_transform_query.iter() {
            let window = windows.get_primary().unwrap();
            let size = bevy::math::Vec2::new(window.width() as f32, window.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = event.position - size / 2.0;

            // apply the camera transform
            let pos_wld = transform.compute_matrix() * p.extend(0.0).extend(1.0);

            ui_state.projected_mouse_position.coord.x = pos_wld.x;
            ui_state.projected_mouse_position.coord.y = pos_wld.y;
        }
    }
}

fn mouse_motion_system(
    mut mouse_motion_event_reader: bevy::app::EventReader<bevy::input::mouse::MouseMotion>,
    mouse_button: Res<bevy::input::Input<bevy::input::mouse::MouseButton>>,
    mut pan_camera_events: ResMut<bevy::app::Events<rgis_camera::PanCameraEvent>>,
) {
    if mouse_button.pressed(bevy::input::mouse::MouseButton::Right) {
        for event in mouse_motion_event_reader.iter() {
            pan_camera_events.send(rgis_camera::PanCameraEvent {
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
    }
}

fn mouse_scroll_system(
    mut mouse_scroll_event_reader: bevy::app::EventReader<bevy::input::mouse::MouseWheel>,
    mut zoom_camera_events: ResMut<bevy::app::Events<rgis_camera::ZoomCameraEvent>>,
) {
    for event in mouse_scroll_event_reader.iter() {
        if event.y > 0. {
            zoom_camera_events.send(rgis_camera::ZoomCameraEvent::zoom_in());
        } else if event.y < 0. {
            zoom_camera_events.send(rgis_camera::ZoomCameraEvent::zoom_out());
        }
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_system(cursor_moved_system.system())
            .add_system(mouse_scroll_system.system())
            .add_system(mouse_motion_system.system());
    }
}
