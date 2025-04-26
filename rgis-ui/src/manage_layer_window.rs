use bevy::color::ColorToComponents;
use bevy_egui::egui;

pub(crate) struct ManageLayerWindow<'a> {
    pub state: &'a mut crate::ManageLayerWindowState,
    pub layers: &'a rgis_layers::Layers,
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub color_events: &'a mut bevy::ecs::event::Events<rgis_events::UpdateLayerColorEvent>,
}

impl ManageLayerWindow<'_> {
    pub(crate) fn render(&mut self) {
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
                        ui.label(format!("EPSG {}", layer.crs_epsg_code));
                        ui.end_row();
                        if layer.geom_type.has_fill() {
                            if let Some(fill) = layer.color.fill {
                                ui.label("Fill color");
                                ui.add(FillColorWidget {
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
                        ui.add(StrokeColorWidget {
                            layer_id,
                            color: layer.color.stroke,
                            color_events: self.color_events,
                        });
                        ui.end_row();
                    });
            });
    }
}

struct StrokeColorWidget<'a> {
    layer_id: rgis_layer_id::LayerId,
    color: bevy::prelude::Color,
    pub color_events: &'a mut bevy::ecs::event::Events<rgis_events::UpdateLayerColorEvent>,
}

impl egui::Widget for StrokeColorWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut old_color = self.color.to_linear().to_f32_array();
        let response = ui.color_edit_button_rgba_unmultiplied(&mut old_color);
        if response.changed() {
            self.color_events
                .send(rgis_events::UpdateLayerColorEvent::Stroke(
                    self.layer_id,
                    bevy::prelude::Color::linear_rgba(
                        old_color[0],
                        old_color[1],
                        old_color[2],
                        old_color[3],
                    ),
                ));
        }
        response
    }
}

struct FillColorWidget<'a> {
    layer_id: rgis_layer_id::LayerId,
    color: bevy::prelude::Color,
    pub color_events: &'a mut bevy::ecs::event::Events<rgis_events::UpdateLayerColorEvent>,
}

impl egui::Widget for FillColorWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut old_color = self.color.to_linear().to_f32_array();
        let response = ui.color_edit_button_rgba_unmultiplied(&mut old_color);
        if response.changed() {
            self.color_events
                .send(rgis_events::UpdateLayerColorEvent::Fill(
                    self.layer_id,
                    bevy::prelude::Color::linear_rgba(
                        old_color[0],
                        old_color[1],
                        old_color[2],
                        old_color[3],
                    ),
                ));
        }
        response
    }
}
