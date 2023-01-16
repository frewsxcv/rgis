use bevy_egui::egui;

pub(crate) struct ChangeCrsWindow<'a, 'w, 's> {
    pub is_visible: &'a mut bool,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub text_field_value: &'a mut String,
    pub change_crs_event_writer:
        &'a mut bevy::ecs::event::EventWriter<'w, 's, rgis_events::ChangeCrsEvent>,
    pub rgis_settings: &'a rgis_settings::RgisSettings,
}

impl<'a, 'w, 's> ChangeCrsWindow<'a, 'w, 's> {
    pub(crate) fn render(&mut self) {
        egui::Window::new("Change CRS")
            .open(self.is_visible)
            .anchor(egui::Align2::LEFT_TOP, [5., 5.])
            .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                let edit_field = ui.text_edit_singleline(self.text_field_value);
                if edit_field.changed() {}
                // TODO: DONT CALL THIS ON EVERY LOOP
                let result = lookup_crs(self.text_field_value);
                let message = match &result {
                    Ok(n) => format!("✅ {n}"),
                    Err(e) => format!("❌ {e:?}"),
                };
                ui.label(message);
                if ui
                    .add_enabled(result.is_ok(), egui::Button::new("Set"))
                    .clicked()
                {
                    self.change_crs_event_writer
                        .send(rgis_events::ChangeCrsEvent {
                            old_crs: self.rgis_settings.target_crs.clone(),
                            new_crs: self.text_field_value.clone(),
                        })
                }
            });
    }
}

#[cfg(target_arch = "wasm32")]
fn lookup_crs(query: &str) -> Result<String, geo_proj_js::Error> {
    geo_proj_js::lookup_crs(query)
}

#[cfg(not(target_arch = "wasm32"))]
fn lookup_crs(query: &str) -> Result<String, geo::algorithm::proj::ProjCreateError> {
    geo::algorithm::proj::Proj::new(query).map(|p| {
        p.proj_info()
            .description
            .unwrap_or_else(|| query.to_string())
    })
}
