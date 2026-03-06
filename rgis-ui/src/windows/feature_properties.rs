use bevy_egui::egui;

pub struct FeaturePropertiesContent<'a> {
    pub layer_name: &'a str,
    pub properties: &'a geo_features::Properties,
}

impl egui::Widget for FeaturePropertiesContent<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label(format!("Layer: {}", self.layer_name));
        ui.add(
            crate::widgets::feature_properties_table::FeaturePropertiesTable {
                properties: self.properties,
            },
        )
    }
}
