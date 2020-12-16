use bevy::prelude::*;
use rgis_camera::Camera;

pub fn system(
    mut cursor_moved_event_reader: Local<EventReader<CursorMoved>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera,)>,
    transform_query: Query<(&Transform,)>,
    mut text_query: Query<&mut Text, With<crate::plugins::ui::PositionText>>,
) {
    for event in cursor_moved_event_reader.iter(&cursor_moved_events) {
        for (camera,) in camera_query.iter() {
            if let Ok((transform,)) = transform_query.get(camera.0) {
                let window = windows.get_primary().unwrap();
                let size = Vec2::new(window.width() as f32, window.height() as f32);

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

    /*
    if mouse_button_input.pressed(MouseButton::Left) {
        println!("left mouse currently pressed");

    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        println!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        println!("left mouse just released");
    }
    */
}
