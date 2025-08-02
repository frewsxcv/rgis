use bevy_egui::egui;

pub struct ManageLayerWindow<'a> {
    pub state: &'a mut crate::ManageLayerWindowState,
    pub layers: &'a rgis_layers::Layers,
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub color_events: &'a mut bevy::ecs::event::Events<rgis_events::UpdateLayerColorEvent>,
}

impl ManageLayerWindow<'_> {
    pub fn render(&mut self) {
        let (true, Some(layer_id)) = (self.state.is_visible, self.state.layer_id) else {
            return;
        };
        let Some(layer) = self.layers.get(layer_id) else {
            bevy::log::warn!(
                "Could not find layer with ID {:?}, closing manage layer window",
                layer_id
            );
            self.state.is_visible = false;
            return;
        };
        egui::Window::new("Manage Layer")
            .open(&mut self.state.is_visible)
            .show(self.egui_ctx, |ui| {
                egui::Grid::new("manage_layer_window_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Name");
                        ui.label(&layer.name);
                        ui.end_row();
                        ui.label("CRS");
                        ui.label(format!("EPSG {}", layer.crs.epsg_code));
                        ui.end_row();
                        if layer.geom_type.has_fill() {
                            if let Some(fill) = layer.color.fill {
                                ui.label("Fill color");
                                ui.add(crate::widgets::fill_color_widget::FillColorWidget {
                                    layer_id,
                                    color: fill,
                                    color_events: self.color_events,
                                });
                                ui.end_row();
                            } else {
                                bevy::log::error!("Expected layer to have a fill color");
                            }
                        }
                        ui.label("Stroke color");
                        ui.add(crate::widgets::stroke_color_widget::StrokeColorWidget {
                            layer_id,
                            color: layer.color.stroke,
                            color_events: self.color_events,
                        });
                        ui.end_row();
                    });
            });
    }
}
