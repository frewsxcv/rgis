use bevy_egui::egui;

const MAX_SIDE_PANEL_WIDTH: f32 = 200.0f32;

pub(crate) struct SidePanel<'a> {
    pub egui_ctx: &'a egui::CtxRef,
    pub state: &'a mut crate::UiState,
    pub layers: &'a rgis_layers::Layers,
    pub toggle_layer_visibility_events:
        &'a mut bevy::app::Events<rgis_events::ToggleLayerVisibilityEvent>,
    pub center_layer_events: &'a mut bevy::app::Events<rgis_events::CenterCameraEvent>,
    pub delete_layer_events: &'a mut bevy::app::Events<rgis_events::DeleteLayer>,
}

impl<'a> SidePanel<'a> {
    pub fn render(&mut self) {
        egui::SidePanel::left("left-side-panel")
            .max_width(MAX_SIDE_PANEL_WIDTH)
            .show(self.egui_ctx, |ui| {
                self.render_layers_window(ui);
            });
    }

    fn render_layers_window(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗺 Layers");
        if ui.button("Add Layer").clicked() {
            self.state.is_add_layer_window_visible = true;
        }

        for layer in &self.layers.data {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                egui::CollapsingHeader::new(layer.name.to_owned())
                    .id_source(layer.id) // Instead of using the layer name as the ID (which is not unique), use the layer ID
                    .show(ui, |ui| {
                        if ui.button("✏ Manage").clicked() {
                            self.state.is_manage_layer_window_visible = true;
                            self.state.managing_layer = Some(layer.id);
                        }

                        if layer.visible {
                            if ui.button("👁 Hide").clicked() {
                                self.toggle_layer_visibility_events
                                    .send(rgis_events::ToggleLayerVisibilityEvent(layer.id));
                            }
                        } else if ui.button("👁 Show").clicked() {
                            self.toggle_layer_visibility_events
                                .send(rgis_events::ToggleLayerVisibilityEvent(layer.id));
                        }

                        if ui.button("🔎 Zoom to extent").clicked() {
                            self.center_layer_events
                                .send(rgis_events::CenterCameraEvent(layer.id))
                        }

                        if ui.button("❌ Remove").clicked() {
                            self.delete_layer_events
                                .send(rgis_events::DeleteLayer(layer.id))
                        }
                    });
            });
        }
    }
}
