use bevy_egui::egui;
use rgis_events::CreateLayerMessage;
use rgis_ui_messages::RenderTextMessage;

pub struct Operation<'w> {
    pub egui_ctx: &'w mut bevy_egui::egui::Context,
    pub state: &'w mut crate::OperationWindowState,
    pub create_layer_event_writer: bevy::ecs::message::MessageWriter<'w, CreateLayerMessage>,
    pub render_message_event_writer: bevy::ecs::message::MessageWriter<'w, RenderTextMessage>,
    pub default_pos: egui::Pos2,
}

impl Operation<'_> {
    pub fn render(&mut self) {
        let Some(ref mut data) = *self.state else {
            return;
        };

        match data.operation.next_action() {
            rgis_geo_ops::Action::Perform => {
                // TODO: perform in background job
                let layer_name = format!("{} of {}", data.operation.name(), data.layer_name);
                let outcome = data.operation.perform(&data.feature_collection);
                let source_crs = match data.source_crs.clone() {
                    Some(crs) => crs,
                    None => {
                        bevy::log::error!("Source CRS is not set for the operation");
                        return;
                    }
                };
                match outcome {
                    Ok(rgis_geo_ops::Outcome::FeatureCollection(feature_collection)) => {
                        self.create_layer_event_writer
                            .write(CreateLayerMessage {
                                feature_collection: std::sync::Arc::new(feature_collection),
                                name: layer_name,
                                source_crs,
                            });
                    }
                    Ok(rgis_geo_ops::Outcome::Text(text)) => {
                        self.render_message_event_writer
                            .write(RenderTextMessage(text));
                    }
                    Err(e) => {
                        bevy::log::error!("Encountered an error during the operation: {}", e);
                    }
                }
                *self.state = None;
            }
            rgis_geo_ops::Action::RenderUi => {
                let mut is_open = true;
                egui::Window::new("Operation")
                    .default_pos(self.default_pos)
                    .open(&mut is_open)
                    .show(self.egui_ctx, |ui| {
                        data.operation.ui(ui, &data.feature_collection);
                    });

                if !is_open {
                    *self.state = None;
                }
            }
        }
    }
}
