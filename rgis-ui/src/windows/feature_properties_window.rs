use bevy_egui::egui;

pub struct FeaturePropertiesWindow<'a> {
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub state: &'a mut crate::FeaturePropertiesWindowState,
    pub layer: &'a rgis_layers::Layer,
}

impl FeaturePropertiesWindow<'_> {
    pub fn render(&mut self) {
        let Some(ref properties) = self.state.properties else {
            return;
        };
        egui::Window::new("Layer Feature Properties")
            .id(egui::Id::new("Layer Feature Properties Window"))
            .open(&mut self.state.is_visible)
            .show(self.egui_ctx, |ui| {
                ui.label(format!("Layer: {}", self.layer.name));
                ui.add(
                    crate::widgets::feature_properties_table::FeaturePropertiesTable { properties },
                )
            });
    }
}
