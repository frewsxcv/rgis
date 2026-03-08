use bevy_egui::egui;

pub struct FeaturePropertiesTable<'a> {
    pub properties: &'a [(String, String)],
}

impl egui::Widget for FeaturePropertiesTable<'_> {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        egui::Grid::new("feature_properties_window_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                for (k, v) in self.properties.iter() {
                    ui.label(k);
                    ui.label(v);
                    ui.end_row();
                }
            })
            .response
    }
}
