use bevy::prelude::*;
use bevy_egui::egui;
use rgis_events::DuplicateLayerMessage;
use rgis_ui_messages::{
    RenameLayerMessage, UpdateLayerColorMessage,
    UpdateLayerPointSizeMessage,
};

pub struct ManageLayer<'a> {
    pub layer_id: rgis_primitives::LayerId,
    pub name: &'a rgis_layers::LayerName,
    pub color: &'a rgis_layers::LayerColor,
    pub point_size: &'a rgis_layers::LayerPointSize,
    pub data: &'a rgis_layers::LayerData,
    pub crs: &'a rgis_layers::LayerCrs,
    pub color_events: &'a mut Messages<UpdateLayerColorMessage>,
    pub point_size_events: &'a mut Messages<UpdateLayerPointSizeMessage>,
    pub duplicate_layer_events: &'a mut Messages<DuplicateLayerMessage>,
    pub rename_events: &'a mut Messages<RenameLayerMessage>,
    pub name_edit_buffer: &'a mut String,
}

impl ManageLayer<'_> {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        let layer_id = self.layer_id;

        egui::Grid::new("manage_layer_window_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Name");
                let response = ui.text_edit_singleline(self.name_edit_buffer);
                if response.lost_focus()
                    && *self.name_edit_buffer != self.name.0
                    && !self.name_edit_buffer.is_empty()
                {
                    self.rename_events.write(RenameLayerMessage(
                        layer_id,
                        self.name_edit_buffer.clone(),
                    ));
                }
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
    }
}
