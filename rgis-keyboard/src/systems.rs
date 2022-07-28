use bevy::prelude::*;

const PAN_AMOUNT: f32 = 15.; // Larger number will pan more

fn process_key_code_input_system(
    keyboard_input: Res<bevy::input::Input<bevy::input::keyboard::KeyCode>>,
    mut pan_camera_events: bevy::ecs::event::EventWriter<rgis_events::PanCameraEvent>,
    // mut zoom_camera_events: bevy::ecs::event::EventWriter<rgis_events::ZoomCameraEvent>,
) {
    for key in keyboard_input.get_just_pressed() {
        match key {
            bevy::input::keyboard::KeyCode::Up => {
                pan_camera_events.send(rgis_events::PanCameraEvent::up(PAN_AMOUNT));
            }
            bevy::input::keyboard::KeyCode::Right => {
                pan_camera_events.send(rgis_events::PanCameraEvent::right(PAN_AMOUNT));
            }
            bevy::input::keyboard::KeyCode::Down => {
                pan_camera_events.send(rgis_events::PanCameraEvent::down(PAN_AMOUNT));
            }
            bevy::input::keyboard::KeyCode::Left => {
                pan_camera_events.send(rgis_events::PanCameraEvent::left(PAN_AMOUNT));
            }
            // bevy::input::keyboard::KeyCode::Minus => {
            //     zoom_camera_events.send(rgis_events::ZoomCameraEvent::zoom_out())
            // }
            // bevy::input::keyboard::KeyCode::Equals => {
            //     if keyboard_input.pressed(bevy::input::keyboard::KeyCode::RShift)
            //         || keyboard_input.pressed(bevy::input::keyboard::KeyCode::LShift)
            //     {
            //         zoom_camera_events.send(rgis_events::ZoomCameraEvent::zoom_in())
            //     }
            // }
            _ => {}
        }
    }
}

pub fn system_set() -> SystemSet {
    SystemSet::new().with_system(process_key_code_input_system)
}
