use bevy::prelude::*;

const PAN_AMOUNT: f32 = 15.; // Larger number will pan more

fn process_key_code_input_system(
    keyboard_input: Res<bevy::input::ButtonInput<bevy::input::keyboard::KeyCode>>,
    mut pan_camera_events: bevy::ecs::event::EventWriter<rgis_events::PanCameraEvent>,
) {
    for key in keyboard_input.get_just_pressed() {
        match key {
            bevy::input::keyboard::KeyCode::ArrowUp => {
                pan_camera_events.send(rgis_events::PanCameraEvent::up(PAN_AMOUNT));
            }
            bevy::input::keyboard::KeyCode::ArrowRight => {
                pan_camera_events.send(rgis_events::PanCameraEvent::right(PAN_AMOUNT));
            }
            bevy::input::keyboard::KeyCode::ArrowDown => {
                pan_camera_events.send(rgis_events::PanCameraEvent::down(PAN_AMOUNT));
            }
            bevy::input::keyboard::KeyCode::ArrowLeft => {
                pan_camera_events.send(rgis_events::PanCameraEvent::left(PAN_AMOUNT));
            }
            _ => {}
        }
    }
}

pub fn configure(app: &mut App) {
    app.add_systems(Update, process_key_code_input_system);
}
