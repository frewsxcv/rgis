use bevy::prelude::*;
use bevy_egui::egui;
use rgis_events::DuplicateLayerMessage;
use rgis_ui_messages::{RenameLayerMessage, UpdateLayerColorMessage, UpdateLayerPointSizeMessage};

pub struct ManageLayer<'a> {
    pub state: &'a mut crate::ManageLayerWindowState,
    pub layers: &'a rgis_layers::Layers,
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub color_events: &'a mut Messages<UpdateLayerColorMessage>,
    pub point_size_events: &'a mut Messages<UpdateLayerPointSizeMessage>,
    pub duplicate_layer_events: &'a mut Messages<DuplicateLayerMessage>,
    pub rename_events: &'a mut Messages<RenameLayerMessage>,
    pub name_edit_buffer: &'a mut String,
    pub name_edit_layer_id: &'a mut Option<rgis_primitives::LayerId>,
}

impl ManageLayer<'_> {
    pub fn render(&mut self) {
        let Some(layer_id) = *self.state else {
            return;
        };
        let Some(layer) = self.layers.get(layer_id) else {
            warn!(
                "Could not find layer with ID {:?}, closing manage layer window",
                layer_id
            );
            *self.state = None;
            return;
        };

        // Initialize or reset the edit buffer when switching layers
        if *self.name_edit_layer_id != Some(layer_id) {
            *self.name_edit_layer_id = Some(layer_id);
            self.name_edit_buffer.clone_from(&layer.name);
        }

        let mut is_open = true;
        egui::Window::new("Manage Layer")
            .open(&mut is_open)
            .show(self.egui_ctx, |ui| {
                egui::Grid::new("manage_layer_window_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Name");
                        let response = ui.text_edit_singleline(self.name_edit_buffer);
                        if response.lost_focus()
                            && *self.name_edit_buffer != layer.name
                            && !self.name_edit_buffer.is_empty()
                        {
                            self.rename_events.write(RenameLayerMessage(
                                layer_id,
                                self.name_edit_buffer.clone(),
                            ));
                        }
                        ui.end_row();
                        ui.label("CRS");
                        match layer.crs.epsg_code {
                            Some(code) => ui.label(format!("EPSG {code}")),
                            None => ui.label("Custom PROJ"),
                        };
                        ui.end_row();
                        if let Some(geom_type) = layer.geom_type() {
                            if geom_type.has_fill() {
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
                            if geom_type.contains(geo_geom_type::GeomType::POINT)
                                || geom_type
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
