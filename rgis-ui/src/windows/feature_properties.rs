use bevy_egui::egui;

pub struct FeatureProperties<'a> {
    pub layer_name: &'a str,
    pub properties: &'a [(String, String)],
}

impl FeatureProperties<'_> {
    pub fn render(&self, ui: &mut egui::Ui) {
        ui.label(format!("Layer: {}", self.layer_name));
        ui.add(
            crate::widgets::feature_properties_table::FeaturePropertiesTable {
                properties: self.properties,
            },
        );
    }
}
