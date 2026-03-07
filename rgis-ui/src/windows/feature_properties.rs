use bevy_egui::egui;

pub struct FeatureProperties<'a> {
    pub layer: &'a rgis_layers::Layer,
    pub properties: &'a geo_features::Properties,
}

impl FeatureProperties<'_> {
    pub fn render(&self, ui: &mut egui::Ui) {
        ui.label(format!("Layer: {}", self.layer.name));
        ui.add(
            crate::widgets::feature_properties_table::FeaturePropertiesTable {
                properties: self.properties,
            },
        );
    }
}
