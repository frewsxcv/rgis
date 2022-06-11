use bevy_egui::egui;

pub(crate) struct ChangeCrsWindow<'a> {
    pub state: &'a mut crate::UiState,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub text_field_value: &'a mut String,
}

impl<'a> ChangeCrsWindow<'a> {
    pub(crate) fn render(&mut self) {
        egui::Window::new("Change CRS")
            .open(&mut self.state.is_change_crs_window_visible)
            .anchor(egui::Align2::LEFT_TOP, [5., 5.])
            .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let edit_field = ui.text_edit_singleline(self.text_field_value);
                    if edit_field.changed() {
                    }
                    ui.label(
                        match geo::algorithm::proj::Proj::new(self.text_field_value) { // DONT CALL THIS ON EVERY LOOP
                            Ok(n) => format!("✅ {:?}", n),
                            Err(e) => format!("❌ {:?}", e),
                        }
                    );
                }
            });
    }
}
