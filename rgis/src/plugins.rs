use bevy::prelude::*;

pub struct KeyboardCameraMover;

impl Plugin for KeyboardCameraMover {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(process_mouse_events.system());
    }
}

fn process_mouse_events(
    keyboard_input: Res<Input<KeyCode>>,
    camera_query: Query<(&crate::Camera,)>,
    mut transform_query: Query<(&mut Transform,)>,
) {
    let pressed = keyboard_input.get_just_pressed().collect::<Vec<_>>();
    if pressed.len() > 0 {
        for (camera,) in camera_query.iter() {
            if let Ok((mut transform,)) = transform_query.get_mut(camera.0) {
                *transform.translation.x_mut() = transform.translation.x() + 5.0;
                println!("transform: {:?}", transform);
            }
        }

        println!("pressed: {:?}", pressed);
    }
}
