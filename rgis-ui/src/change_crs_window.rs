use bevy_egui::egui;

pub(crate) struct ChangeCrsWindow<'a, 'w, 's> {
    pub is_visible: &'a mut bool,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub text_field_value: &'a mut String,
    pub change_crs_event_writer:
        &'a mut bevy::ecs::event::EventWriter<'w, 's, rgis_events::ChangeCrsEvent>,
}

impl<'a, 'w, 's> ChangeCrsWindow<'a, 'w, 's> {
    pub(crate) fn render(&mut self) {
        egui::Window::new("Change CRS")
            .open(self.is_visible)
            .anchor(egui::Align2::LEFT_TOP, [5., 5.])
            .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let edit_field = ui.text_edit_singleline(self.text_field_value);
                    if edit_field.changed() {}
                    // TODO: DONT CALL THIS ON EVERY LOOP
                    let proj_result = geo::algorithm::proj::Proj::new(self.text_field_value);
                    let message = match &proj_result {
                        Ok(n) => format!(
                            "✅ {}",
                            n.proj_info()
                                .description
                                .unwrap_or_else(|| self.text_field_value.to_string())
                        ),
                        Err(e) => format!("❌ {:?}", e),
                    };
                    ui.label(message);
                    if ui
                        .add_enabled(proj_result.is_ok(), egui::Button::new("Set"))
                        .clicked()
                    {
                        self.change_crs_event_writer
                            .send(rgis_events::ChangeCrsEvent {
                                crs: self.text_field_value.clone(),
                            })
                    }
                }
            });
    }
}
