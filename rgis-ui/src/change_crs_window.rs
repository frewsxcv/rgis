use bevy_egui::egui;
use std::str::FromStr;

pub(crate) struct ChangeCrsWindow<'a, 'w> {
    pub is_visible: &'a mut bool,
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub text_field_value: &'a mut String,
    pub change_crs_event_writer:
        &'a mut bevy::ecs::event::EventWriter<'w, rgis_events::ChangeCrsEvent>,
    pub target_crs: rgis_crs::TargetCrs,
    pub crs_input_outcome: &'a mut Option<crate::widgets::crs_input::Outcome>,
}

impl ChangeCrsWindow<'_, '_> {
    pub(crate) fn render(&mut self) {
        egui::Window::new("Change CRS")
            .open(self.is_visible)
            .show(self.egui_ctx, |ui| {
                ui.add(crate::widgets::CrsInput::new(
                    self.text_field_value,
                    self.crs_input_outcome,
                ));
                let (is_ok, op_handle) = match self.crs_input_outcome {
                    Some(Ok((_, op_handle))) => (true, Some(op_handle)),
                    _ => (false, None),
                };
                if ui.add_enabled(is_ok, egui::Button::new("Set")).clicked() {
                    let Ok(epsg_code) = u16::from_str(self.text_field_value) else {
                        // TODO: show error message to the user instead of logging an error
                        bevy::log::error!("Could not parse u16 value");
                        return;
                    };
                    self.change_crs_event_writer
                        .write(rgis_events::ChangeCrsEvent {
                            old: self.target_crs.0,
                            new: rgis_primitives::Crs {
                                epsg_code,
                                op_handle: *op_handle.unwrap(),
                            },
                        });
                }
            });
    }
}
