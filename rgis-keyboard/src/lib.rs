use bevy::ecs::IntoSystem;

const PAN_AMOUNT: f32 = 15.; // Larger number will pan more

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.add_system(process_key_code_input_system.system());
    }
}

fn process_key_code_input_system(
    keyboard_input: bevy::ecs::Res<bevy::input::Input<bevy::input::keyboard::KeyCode>>,
    mut pan_camera_events: bevy::ecs::ResMut<bevy::app::Events<rgis_camera::PanCameraEvent>>,
    mut zoom_camera_events: bevy::ecs::ResMut<bevy::app::Events<rgis_camera::ZoomCameraEvent>>,
) {
    for key in keyboard_input.get_just_pressed() {
        match key {
            bevy::input::keyboard::KeyCode::Up => {
                pan_camera_events.send(rgis_camera::PanCameraEvent::up(PAN_AMOUNT));
            }
            bevy::input::keyboard::KeyCode::Right => {
                pan_camera_events.send(rgis_camera::PanCameraEvent::right(PAN_AMOUNT));
            }
            bevy::input::keyboard::KeyCode::Down => {
                pan_camera_events.send(rgis_camera::PanCameraEvent::down(PAN_AMOUNT));
            }
            bevy::input::keyboard::KeyCode::Left => {
                pan_camera_events.send(rgis_camera::PanCameraEvent::left(PAN_AMOUNT));
            }
            bevy::input::keyboard::KeyCode::Minus => {
                zoom_camera_events.send(rgis_camera::ZoomCameraEvent::zoom_out())
            }
            bevy::input::keyboard::KeyCode::Equals => {
                if keyboard_input.pressed(bevy::input::keyboard::KeyCode::RShift)
                    || keyboard_input.pressed(bevy::input::keyboard::KeyCode::LShift)
                {
                    zoom_camera_events.send(rgis_camera::ZoomCameraEvent::zoom_in())
                }
            }
            _ => {}
        }
    }
}
