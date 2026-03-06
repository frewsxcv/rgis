use bevy::prelude::*;

/// Resource holding the configured pan amount for keyboard input.
#[derive(Resource)]
pub struct PanAmount(pub f32);

fn process_key_code_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut pan_camera_events: MessageWriter<rgis_camera_messages::PanCameraMessage>,
    pan_amount: Res<PanAmount>,
) {
    let pan_amount = pan_amount.0;
    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::ArrowUp => {
                pan_camera_events.write(rgis_camera_messages::PanCameraMessage::up(pan_amount));
            }
            KeyCode::ArrowRight => {
                pan_camera_events.write(rgis_camera_messages::PanCameraMessage::right(pan_amount));
            }
            KeyCode::ArrowDown => {
                pan_camera_events.write(rgis_camera_messages::PanCameraMessage::down(pan_amount));
            }
            KeyCode::ArrowLeft => {
                pan_camera_events.write(rgis_camera_messages::PanCameraMessage::left(pan_amount));
            }
            _ => {}
        }
    }
}

pub fn configure(app: &mut App) {
    app.add_systems(Update, process_key_code_input_system);
}
