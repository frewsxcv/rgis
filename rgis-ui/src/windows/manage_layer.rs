use bevy_egui::egui;

pub enum ManageLayerAction {
    UpdateFillColor(rgis_primitives::LayerId, [f32; 4]),
    UpdateStrokeColor(rgis_primitives::LayerId, [f32; 4]),
    UpdatePointSize(rgis_primitives::LayerId, f32),
    DuplicateLayer(rgis_primitives::LayerId),
}

pub struct ManageLayerData<'a> {
    pub layer_id: rgis_primitives::LayerId,
    pub name: &'a str,
    pub epsg_code: Option<u16>,
    pub geom_type: Option<geo_geom_type::GeomType>,
    pub fill_color: Option<[f32; 4]>,
    pub stroke_color: [f32; 4],
    pub point_size: f32,
}

pub struct ManageLayerContent<'a> {
    pub data: ManageLayerData<'a>,
}

impl ManageLayerContent<'_> {
    pub fn show(self, ui: &mut egui::Ui) -> Vec<ManageLayerAction> {
        let mut actions = vec![];
        let data = &self.data;

        egui::Grid::new("manage_layer_window_grid")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label(data.name);
                ui.end_row();
                ui.label("CRS");
                match data.epsg_code {
                    Some(code) => ui.label(format!("EPSG {code}")),
                    None => ui.label("Custom PROJ"),
                };
                ui.end_row();
                if let Some(geom_type) = data.geom_type {
                    if geom_type.has_fill() {
                        if let Some(fill) = data.fill_color {
                            ui.label("Fill color");
                            let mut color = fill;
                            let response =
                                ui.color_edit_button_rgba_unmultiplied(&mut color);
                            if response.changed() {
                                actions.push(ManageLayerAction::UpdateFillColor(
                                    data.layer_id,
                                    color,
                                ));
                            }
                            ui.end_row();
                        } else {
                            bevy::log::error!("Expected layer to have a fill color");
                        }
                    }
                    ui.label("Stroke color");
                    let mut color = data.stroke_color;
                    let response = ui.color_edit_button_rgba_unmultiplied(&mut color);
                    if response.changed() {
                        actions.push(ManageLayerAction::UpdateStrokeColor(
                            data.layer_id,
                            color,
                        ));
                    }
                    ui.end_row();
                    if geom_type.contains(geo_geom_type::GeomType::POINT)
                        || geom_type.contains(geo_geom_type::GeomType::MULTI_POINT)
                    {
                        ui.label("Point size");
                        let mut size = data.point_size;
                        let response = ui.add(egui::Slider::new(&mut size, 1.0..=50.0));
                        if response.changed() {
                            actions.push(ManageLayerAction::UpdatePointSize(
                                data.layer_id,
                                size,
                            ));
                        }
                        ui.end_row();
                    }
                }
            });
        ui.separator();
        if ui.button("Duplicate").clicked() {
            actions.push(ManageLayerAction::DuplicateLayer(data.layer_id));
        }

        actions
    }
}
