use bevy_egui::egui;

pub struct FeatureProperties<'a> {
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub state: &'a mut crate::FeaturePropertiesWindowState,
    pub layer_name: &'a str,
}

impl FeatureProperties<'_> {
    pub fn render(&mut self) {
        let Some(ref data) = *self.state else {
            return;
        };

        let properties = &data.properties;
        let mut is_open = true;
        egui::Window::new("Layer Feature Properties")
            .id(egui::Id::new("Layer Feature Properties Window"))
            .open(&mut is_open)
            .show(self.egui_ctx, |ui| {
                ui.label(format!("Layer: {}", self.layer_name));
                ui.add(
                    crate::widgets::feature_properties_table::FeaturePropertiesTable { properties },
                )
            });

        if !is_open {
            *self.state = None;
        }
    }
}
