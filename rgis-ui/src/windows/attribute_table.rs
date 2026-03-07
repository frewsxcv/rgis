use bevy_egui::egui;

pub struct AttributeTable<'a> {
    pub layer: &'a rgis_layers::Layer,
}

impl AttributeTable<'_> {
    pub fn render(&self, ui: &mut egui::Ui) {
        let Some(fc) = self.layer.unprojected_feature_collection() else {
            ui.label("No vector data available.");
            return;
        };

        let Some(ref record_batch) = fc.properties else {
            ui.label("No attributes available.");
            return;
        };

        let schema = record_batch.schema();
        let fields = schema.fields();
        let num_rows = record_batch.num_rows();

        ui.label(format!(
            "{} features, {} fields",
            num_rows,
            fields.len()
        ));
        ui.separator();

        let num_columns = fields.len() + 1; // +1 for row number
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Grid::new("attribute_table_grid")
                    .num_columns(num_columns)
                    .striped(true)
                    .show(ui, |ui| {
                        // Header row
                        ui.strong("#");
                        for field in fields.iter() {
                            ui.strong(field.name());
                        }
                        ui.end_row();

                        // Data rows
                        for row in 0..num_rows {
                            ui.label(row.to_string());
                            let props = geo_features::properties_for_row(record_batch, row);
                            for (_key, value) in &props {
                                ui.label(value);
                            }
                            ui.end_row();
                        }
                    });
            });
    }
}
