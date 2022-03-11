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
    pub thread_pool: &'a bevy::tasks::AsyncComputeTaskPool,
    pub opened_file_bytes_sender: &'a crate::OpenedFileBytesSender,
}

impl<'a> SidePanel<'a> {
    pub fn render(&mut self) {
        egui::SidePanel::left("left-side-panel")
            .max_width(MAX_SIDE_PANEL_WIDTH)
            .show(self.egui_ctx, |ui| {
                self.render_layers_window(ui);
            });
    }

    fn open_geojson_layer(&mut self) {
        let sender = self.opened_file_bytes_sender.clone();
        self.thread_pool
            .spawn(async move {
                let task = rfd::AsyncFileDialog::new().pick_file();
                let file_handle = task.await;
                if let Some(n) = file_handle {
                    sender.send((n.file_name(), n.read().await)).await.unwrap();
                }
            })
            .detach();
    }

    fn render_layers_window(&mut self, ui: &mut egui::Ui) {
        ui.heading("🗺 Layers");
        if ui.button("Add GeoJSON Layer").clicked() {
            self.open_geojson_layer()
        }

        for layer in &self.layers.data {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                egui::CollapsingHeader::new(layer.name.to_owned())
                    .id_source(layer.id) // Instead of using the layer name as the ID (which is not unique), use the layer ID
                    .show(ui, |ui| {
                        if ui.button("✏ Manage").clicked() {
                            self.state.layer_window_visible = true;
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
