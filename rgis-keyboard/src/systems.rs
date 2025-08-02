use bevy::prelude::*;

const PAN_AMOUNT: f32 = 15.; // Larger number will pan more

fn process_key_code_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut pan_camera_events: EventWriter<rgis_events::PanCameraEvent>,
) {
    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::ArrowUp => {
                pan_camera_events.write(rgis_events::PanCameraEvent::up(PAN_AMOUNT));
            }
            KeyCode::ArrowRight => {
                pan_camera_events.write(rgis_events::PanCameraEvent::right(PAN_AMOUNT));
            }
            KeyCode::ArrowDown => {
                pan_camera_events.write(rgis_events::PanCameraEvent::down(PAN_AMOUNT));
            }
            KeyCode::ArrowLeft => {
                pan_camera_events.write(rgis_events::PanCameraEvent::left(PAN_AMOUNT));
            }
            _ => {}
        }
    }
}

pub fn configure(app: &mut App) {
    app.add_systems(Update, process_key_code_input_system);
}
