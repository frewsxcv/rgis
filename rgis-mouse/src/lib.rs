use bevy::ecs::query::With;
use bevy::ecs::system::{IntoSystem, Query, Res, ResMut};

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
    mut cursor_moved_event_reader: bevy::app::EventReader<bevy::window::CursorMoved>,
    windows: Res<bevy::window::Windows>,
    camera_transform_query: Query<
        &bevy::transform::components::Transform,
        With<rgis_camera::Camera2d>,
    >,
    mut mouse_position: ResMut<MousePos>,
) {
    for event in cursor_moved_event_reader.iter() {
        for transform in camera_transform_query.iter() {
            mouse_position.projected = screen_coords_to_geo_coords(
                event.position,
                transform,
                windows.get_primary().unwrap(),
            );
        }
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
    mut mouse_motion_event_reader: bevy::app::EventReader<bevy::input::mouse::MouseMotion>,
    mouse_button: Res<bevy::input::Input<bevy::input::mouse::MouseButton>>,
    mut pan_camera_events: bevy::app::EventWriter<rgis_events::PanCameraEvent>,
) {
    if mouse_button.pressed(bevy::input::mouse::MouseButton::Right) {
        for event in mouse_motion_event_reader.iter() {
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
    }
}

fn mouse_scroll_system(
    mut mouse_scroll_event_reader: bevy::app::EventReader<bevy::input::mouse::MouseWheel>,
    mut zoom_camera_events: bevy::app::EventWriter<rgis_events::ZoomCameraEvent>,
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
        app.add_system(cursor_moved_system.system())
            .add_system(mouse_scroll_system.system())
            .add_system(mouse_motion_system.system())
            .insert_resource(MousePos {
                projected: Xy { x: 0., y: 0. },
            });
    }
}
