use bevy_egui::egui;

pub(crate) struct OperationWindow<'w> {
    pub egui_ctx: &'w mut bevy_egui::egui::Context,
    pub state: &'w mut crate::OperationWindowState,
    pub create_layer_event_writer: bevy::ecs::event::EventWriter<'w, rgis_events::CreateLayerEvent>,
    pub render_message_event_writer:
        bevy::ecs::event::EventWriter<'w, rgis_events::RenderMessageEvent>,
}

impl OperationWindow<'_> {
    pub(crate) fn render(&mut self) {
        if !self.state.is_visible {
            self.state.operation = None;
            return;
        }
        let Some(ref mut operation) = self.state.operation else {
            return;
        };
        match operation.next_action() {
            rgis_geo_ops::Action::Perform => {
                // TODO: perform in background job
                let outcome = operation.perform(self.state.feature_collection.clone());
                match outcome {
                    Ok(rgis_geo_ops::Outcome::FeatureCollection(feature_collection)) => {
                        self.create_layer_event_writer
                            .write(rgis_events::CreateLayerEvent {
                                feature_collection,
                                name: "FOOOOO".into(),      // FIXME
                                source_crs_epsg_code: 4326, // FIXME
                            });
                    }
                    Ok(rgis_geo_ops::Outcome::Text(text)) => {
                        self.render_message_event_writer
                            .write(rgis_events::RenderMessageEvent(text));
                    }
                    Err(e) => {
                        bevy::log::error!("Encountered an error during the operation: {}", e);
                    }
                }
                self.state.is_visible = false;
            }
            rgis_geo_ops::Action::RenderUi => {
                egui::Window::new("Operation")
                    .open(&mut self.state.is_visible)
                    .show(self.egui_ctx, |ui| {
                        operation.ui(ui, &self.state.feature_collection);
                    });
            }
        }
    }
}
