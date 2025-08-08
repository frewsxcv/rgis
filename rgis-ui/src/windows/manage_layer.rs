use bevy::prelude::*;
use bevy_egui::egui;
use rgis_layer_events::DuplicateLayerEvent;
use rgis_ui_events::{UpdateLayerColorEvent, UpdateLayerPointSizeEvent};

pub struct ManageLayer<'a> {
    pub state: &'a mut crate::ManageLayerWindowState,
    pub layers: &'a rgis_layers::Layers,
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub color_events: &'a mut Events<UpdateLayerColorEvent>,
    pub point_size_events: &'a mut Events<UpdateLayerPointSizeEvent>,
    pub duplicate_layer_events: &'a mut Events<DuplicateLayerEvent>,
}

impl ManageLayer<'_> {
    pub fn render(&mut self) {
        let (true, Some(layer_id)) = (self.state.is_visible, self.state.layer_id) else {
            return;
        };
        let Some(layer) = self.layers.get(layer_id) else {
            warn!(
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
                                ui.add(crate::widgets::fill_color::FillColor {
                                    layer_id,
                                    color: fill,
                                    color_events: self.color_events,
                                });
                                ui.end_row();
                            } else {
                                error!("Expected layer to have a fill color");
                            }
                        }
                        ui.label("Stroke color");
                        ui.add(crate::widgets::stroke_color::StrokeColor {
                            layer_id,
                            color: layer.color.stroke,
                            color_events: self.color_events,
                        });
                        ui.end_row();
                        if layer.geom_type.contains(geo_geom_type::GeomType::POINT)
                            || layer
                                .geom_type
                                .contains(geo_geom_type::GeomType::MULTI_POINT)
                        {
                            ui.label("Point size");
                            ui.add(crate::widgets::point_size::PointSize {
                                layer_id,
                                size: layer.point_size,
                                size_events: self.point_size_events,
                            });
                            ui.end_row();
                        }
                    });
                ui.separator();
                if ui.button("Duplicate").clicked() {
                    self.duplicate_layer_events
                        .send(DuplicateLayerEvent(layer_id));
                }
            });
    }
}
