use bevy_egui::egui;

const MAX_SIDE_PANEL_WIDTH: f32 = 200.0f32;

pub struct SidePanel<'a> {
    pub egui_ctx: &'a egui::CtxRef,
    pub ui_state: &'a mut crate::UiState,
    pub rgis_layers_resource: &'a rgis_layers::RgisLayersResource,
    pub toggle_events: &'a mut bevy::app::Events<rgis_layers::ToggleLayerVisibility>,
    pub toggle_material_events: &'a mut bevy::app::Events<rgis_renderer::ToggleMaterialEvent>,
    pub center_layer_events: &'a mut bevy::app::Events<rgis_renderer::CenterCameraEvent>,
}

impl<'a> SidePanel<'a> {
    pub fn render(&mut self) {
        egui::SidePanel::left("left-side-panel")
            .max_width(MAX_SIDE_PANEL_WIDTH)
            .show(self.egui_ctx, |ui| {
                self.render_mouse_position_window(ui);
                self.render_layers_window(ui);
            });
    }

    fn render_mouse_position_window(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("🖱 Mouse Position", |ui| {
            let mut unprojected = self.ui_state.projected_mouse_position.clone();
            unprojected.reproject(&self.ui_state.source_srs);

            ui.label(format!("Source CRS: {}", self.ui_state.source_srs));
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(format!("X: {}", unprojected.coord.x));
                ui.label(format!("Y: {}", unprojected.coord.y));
            });

            ui.label(format!("Target CRS: {}", self.ui_state.target_srs));
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.label(format!("X: {}", self.ui_state.projected_mouse_position.coord.x));
                ui.label(format!("Y: {}", self.ui_state.projected_mouse_position.coord.y));
            });
        });
    }

    fn render_layers_window(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("🗺 Layers", |ui| {
            let rgis_layers_resource = match self.rgis_layers_resource.read() {
                Ok(r) => r,
                Err(_) => {
                    // TODO log failure
                    return;
                }
            };
            for layer in &rgis_layers_resource.data {
                egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.collapsing(layer.name.to_string(), |ui| {
                        if ui.button("✏ Manage").clicked() {
                            self.ui_state.layer_window_visible = true;
                            self.ui_state.managing_layer = Some(layer.id);
                        }

                        if layer.visible {
                            if ui.button("👁 Hide").clicked() {
                                self.toggle_events.send(rgis_layers::ToggleLayerVisibility(layer.id));
                                self.toggle_material_events
                                    .send(rgis_renderer::ToggleMaterialEvent::Hide(layer.id));
                            }
                        } else {
                            if ui.button("👁 Show").clicked() {
                                self.toggle_events.send(rgis_layers::ToggleLayerVisibility(layer.id));
                                self.toggle_material_events
                                    .send(rgis_renderer::ToggleMaterialEvent::Show(layer.id));
                            }
                        }

                        if ui.button("🔎 Zoom to extent").clicked() {
                            self.center_layer_events.send(rgis_renderer::CenterCameraEvent(layer.id))
                        }
                    });
                });
            }
        });
    }
}

