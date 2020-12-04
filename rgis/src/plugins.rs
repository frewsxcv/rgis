use bevy::prelude::*;

pub struct KeyboardCameraMover;

impl Plugin for KeyboardCameraMover {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(process_mouse_events.system());
    }
}

const PAN_AMOUNT: f32 = 5.; // Larger number will pan more
const ZOOM_AMOUNT: f32 = 10. / 9.; // Larger number will zoom more

fn process_mouse_events(
    keyboard_input: Res<Input<KeyCode>>,
    camera_query: Query<(&crate::Camera,)>,
    mut transform_query: Query<(&mut Transform,)>,
) {
    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::Up => pan_y(PAN_AMOUNT, &camera_query, &mut transform_query),
            KeyCode::Right => pan_x(PAN_AMOUNT, &camera_query, &mut transform_query),
            KeyCode::Down => pan_y(-PAN_AMOUNT, &camera_query, &mut transform_query),
            KeyCode::Left => pan_x(-PAN_AMOUNT, &camera_query, &mut transform_query),
            KeyCode::Minus => zoom(1. / ZOOM_AMOUNT, &camera_query, &mut transform_query),
            KeyCode::Equals => {
                if keyboard_input.pressed(KeyCode::RShift)
                    || keyboard_input.pressed(KeyCode::LShift)
                {
                    zoom(ZOOM_AMOUNT, &camera_query, &mut transform_query);
                }
            }
            _ => {}
        }
    }
}

fn pan_x(
    amount: f32,
    camera_query: &Query<(&crate::Camera,)>,
    transform_query: &mut Query<(&mut Transform,)>,
) {
    for (camera,) in camera_query.iter() {
        if let Ok((mut transform,)) = transform_query.get_mut(camera.0) {
            *transform.translation.x_mut() =
                transform.translation.x() + (amount * transform.scale.x());
        }
    }
}

fn pan_y(
    amount: f32,
    camera_query: &Query<(&crate::Camera,)>,
    transform_query: &mut Query<(&mut Transform,)>,
) {
    for (camera,) in camera_query.iter() {
        if let Ok((mut transform,)) = transform_query.get_mut(camera.0) {
            *transform.translation.y_mut() =
                transform.translation.y() + (amount * transform.scale.x());
        }
    }
}

fn zoom(
    amount: f32,
    camera_query: &Query<(&crate::Camera,)>,
    transform_query: &mut Query<(&mut Transform,)>,
) {
    for (camera,) in camera_query.iter() {
        if let Ok((mut transform,)) = transform_query.get_mut(camera.0) {
            transform.scale = transform.scale * Vec3::new(1. / amount, 1. / amount, 1.);
        }
    }
}
