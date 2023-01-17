use bevy_egui::egui;

pub(crate) struct ChangeCrsWindow<'a, 'w, 's> {
    pub is_visible: &'a mut bool,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub text_field_value: &'a mut String,
    pub change_crs_event_writer:
        &'a mut bevy::ecs::event::EventWriter<'w, 's, rgis_events::ChangeCrsEvent>,
    pub rgis_settings: &'a rgis_settings::RgisSettings,
    pub crs_input_outcome: &'a mut Option<crate::widgets::crs_input::Outcome>,
}

impl<'a, 'w, 's> ChangeCrsWindow<'a, 'w, 's> {
    pub(crate) fn render(&mut self) {
        egui::Window::new("Change CRS")
            .open(self.is_visible)
            .anchor(egui::Align2::LEFT_TOP, [5., 5.])
            .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                ui.add(crate::widgets::CrsInput::new(
                    self.text_field_value,
                    self.crs_input_outcome,
                ));
                let is_ok = self
                    .crs_input_outcome
                    .as_ref()
                    .map(|n| n.is_ok())
                    .unwrap_or(false);
                if ui.add_enabled(is_ok, egui::Button::new("Set")).clicked() {
                    self.change_crs_event_writer
                        .send(rgis_events::ChangeCrsEvent {
                            old_crs: self.rgis_settings.target_crs.clone(),
                            new_crs: self.text_field_value.clone(),
                        })
                }
            });
    }
}
