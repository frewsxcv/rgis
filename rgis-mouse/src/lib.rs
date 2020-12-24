use bevy::ecs::IntoSystem;

fn cursor_moved_system(
    mut cursor_moved_event_reader: bevy::ecs::Local<
        bevy::app::EventReader<bevy::window::CursorMoved>,
    >,
    cursor_moved_events: bevy::ecs::Res<bevy::app::Events<bevy::window::CursorMoved>>,
    windows: bevy::ecs::Res<bevy::window::Windows>,
    camera_query: bevy::ecs::Query<(&rgis_camera::Camera,)>,
    transform_query: bevy::ecs::Query<(&bevy::transform::components::Transform,)>,
    mut text_query: bevy::ecs::Query<
        &mut bevy::ui::widget::Text,
        bevy::ecs::With<rgis_ui::PositionText>,
    >,
) {
    for event in cursor_moved_event_reader.iter(&cursor_moved_events) {
        for (camera,) in camera_query.iter() {
            if let Ok((transform,)) = transform_query.get(camera.0) {
                let window = windows.get_primary().unwrap();
                let size = bevy::math::Vec2::new(window.width() as f32, window.height() as f32);

                // the default orthographic projection is in pixels from the center;
                // just undo the translation
                let p = event.position - size / 2.0;

                // apply the camera transform
                let pos_wld = transform.compute_matrix() * p.extend(0.0).extend(1.0);

                for mut text in text_query.iter_mut() {
                    text.value = format!("Lng: {}\nLat: {}", pos_wld.x, pos_wld.y);
                }
            }
        }
    }
}

fn mouse_motion_system(
    mut mouse_motion_event_reader: bevy::ecs::Local<
        bevy::app::EventReader<bevy::input::mouse::MouseMotion>,
    >,
    mouse_motion_events: bevy::ecs::Res<bevy::app::Events<bevy::input::mouse::MouseMotion>>,
    mouse_button: bevy::ecs::Res<bevy::input::Input<bevy::input::mouse::MouseButton>>,
    mut pan_camera_events: bevy::ecs::ResMut<bevy::app::Events<rgis_camera::PanCameraEvent>>,
) {
    if mouse_button.pressed(bevy::input::mouse::MouseButton::Right) {
        for event in mouse_motion_event_reader.iter(&mouse_motion_events) {
            pan_camera_events.send(rgis_camera::PanCameraEvent {
                x: event.delta.x,
                y: event.delta.y,
            });
        }
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_system(cursor_moved_system.system())
            .add_system(mouse_motion_system.system());
    }
}
