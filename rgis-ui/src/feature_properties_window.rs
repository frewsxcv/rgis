use bevy_egui::egui;

pub(crate) struct FeaturePropertiesWindow<'a> {
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub state: &'a mut crate::FeaturePropertiesWindowState,
}

impl<'a> FeaturePropertiesWindow<'a> {
    pub(crate) fn render(&mut self) {
        if let Some(ref properties) = self.state.properties {
            egui::Window::new("Message Window")
                .id(egui::Id::new("Message window"))
                .open(&mut self.state.is_visible)
                .anchor(egui::Align2::CENTER_CENTER, [0., 0.])
                .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                    egui::Grid::new("feature_properties_window_grid")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Key");
                            ui.label("Value");
                            ui.end_row();
                            for (k, v) in properties.iter() {
                                ui.label(k);
                                ui.label(format!("{:?}", v));
                                ui.end_row();
                            }
                        });
                });
        }
    }
}
