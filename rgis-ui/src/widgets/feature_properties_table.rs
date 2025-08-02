use bevy_egui::egui;

pub struct FeaturePropertiesTable<'a> {
    pub properties: &'a geo_features::Properties,
}

impl egui::Widget for FeaturePropertiesTable<'_> {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        egui::Grid::new("feature_properties_window_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                let mut sorted = self.properties.iter().collect::<Vec<_>>();
                sorted.sort_unstable_by_key(|n| n.0);
                for (k, v) in sorted.iter() {
                    ui.label(*k);
                    ui.label(format!("{v:?}"));
                    ui.end_row();
                }
            })
            .response
    }
}
