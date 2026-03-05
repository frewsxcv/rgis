use bevy::prelude::*;
use bevy_egui::egui;
use rgis_layer_messages::DuplicateLayerMessage;
use rgis_ui_messages::{UpdateLayerColorMessage, UpdateLayerPointSizeMessage};

pub struct ManageLayer<'a> {
    pub state: &'a mut crate::ManageLayerWindowState,
    pub layer_id: rgis_primitives::LayerId,
    pub name: &'a rgis_layers::LayerName,
    pub color: &'a rgis_layers::LayerColor,
    pub point_size: &'a rgis_layers::LayerPointSize,
    pub data: &'a rgis_layers::LayerData,
    pub crs: &'a rgis_layers::LayerCrs,
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub color_events: &'a mut Messages<UpdateLayerColorMessage>,
    pub point_size_events: &'a mut Messages<UpdateLayerPointSizeMessage>,
    pub duplicate_layer_events: &'a mut Messages<DuplicateLayerMessage>,
}

impl ManageLayer<'_> {
    pub fn render(&mut self) {
        let layer_id = self.layer_id;
        let mut is_open = true;
        egui::Window::new("Manage Layer")
            .open(&mut is_open)
            .show(self.egui_ctx, |ui| {
                egui::Grid::new("manage_layer_window_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Name");
                        ui.label(&self.name.0);
                        ui.end_row();
                        ui.label("CRS");
                        match self.crs.0.epsg_code {
                            Some(code) => ui.label(format!("EPSG {code}")),
                            None => ui.label("Custom PROJ"),
                        };
                        ui.end_row();
                        if let Some(geom_type) = self.data.geom_type() {
                            if geom_type.has_fill() {
                                if let Some(fill) = self.color.fill {
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
                                color: self.color.stroke,
                                color_events: self.color_events,
                            });
                            ui.end_row();
                            if geom_type.contains(geo_geom_type::GeomType::POINT)
                                || geom_type
                                    .contains(geo_geom_type::GeomType::MULTI_POINT)
                            {
                                ui.label("Point size");
                                ui.add(crate::widgets::point_size::PointSize {
                                    layer_id,
                                    size: self.point_size.0,
                                    size_events: self.point_size_events,
                                });
                                ui.end_row();
                            }
                        }
                    });
                ui.separator();
                if ui.button("Duplicate").clicked() {
                    self.duplicate_layer_events
                        .write(DuplicateLayerMessage(layer_id));
                }
            });

        if !is_open {
            *self.state = None;
        }
    }
}
