use bevy_egui::egui;

const MAX_SIDE_PANEL_WIDTH: f32 = 200.0f32;

#[derive(bevy::ecs::system::SystemParam)]
pub struct Events<'w, 's> {
    toggle_layer_visibility_event_writer:
        bevy::app::EventWriter<'w, 's, rgis_events::ToggleLayerVisibilityEvent>,
    center_layer_event_writer: bevy::app::EventWriter<'w, 's, rgis_events::CenterCameraEvent>,
    delete_layer_event_writer: bevy::app::EventWriter<'w, 's, rgis_events::DeleteLayer>,
}

pub(crate) struct SidePanel<'a, 'w, 's> {
    pub egui_ctx: &'a egui::Context,
    pub state: &'a mut crate::UiState,
    pub layers: &'a rgis_layers::Layers,
    pub events: &'a mut Events<'w, 's>,
}

impl<'a, 'w, 's> SidePanel<'a, 'w, 's> {
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

        for layer in self.layers.data.iter().rev() {
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
                                self.events
                                    .toggle_layer_visibility_event_writer
                                    .send(rgis_events::ToggleLayerVisibilityEvent(layer.id));
                            }
                        } else if ui.button("👁 Show").clicked() {
                            self.events
                                .toggle_layer_visibility_event_writer
                                .send(rgis_events::ToggleLayerVisibilityEvent(layer.id));
                        }

                        if ui.button("🔎 Zoom to extent").clicked() {
                            self.events
                                .center_layer_event_writer
                                .send(rgis_events::CenterCameraEvent(layer.id))
                        }

                        if ui.button("❌ Remove").clicked() {
                            self.events
                                .delete_layer_event_writer
                                .send(rgis_events::DeleteLayer(layer.id))
                        }
                    });
            });
        }
    }
}
