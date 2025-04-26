use bevy_egui::egui;

pub(crate) struct FeaturePropertiesWindow<'a> {
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub state: &'a mut crate::FeaturePropertiesWindowState,
    pub layer: &'a rgis_layers::Layer,
}

impl FeaturePropertiesWindow<'_> {
    pub(crate) fn render(&mut self) {
        let Some(ref properties) = self.state.properties else {
            return;
        };
        egui::Window::new("Layer Feature Properties")
            .id(egui::Id::new("Layer Feature Properties Window"))
            .open(&mut self.state.is_visible)
            .show(self.egui_ctx, |ui| {
                ui.label(format!("Layer: {}", self.layer.name));
                ui.add(FeaturePropertiesTable { properties })
            });
    }
}

struct FeaturePropertiesTable<'a> {
    properties: &'a geo_features::Properties,
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
