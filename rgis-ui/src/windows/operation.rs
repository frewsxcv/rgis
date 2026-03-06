use bevy_egui::egui;

pub enum OperationAction {
    CreateLayer {
        feature_collection:
            geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
        name: String,
        source_crs: rgis_primitives::Crs,
    },
    RenderMessage(String),
}

pub struct Operation<'a> {
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub state: &'a mut crate::OperationWindowState,
}

impl Operation<'_> {
    pub fn render(&mut self) -> Option<OperationAction> {
        let Some(ref mut data) = *self.state else {
            return None;
        };

        match data.operation.next_action() {
            rgis_geo_ops::Action::Perform => {
                // TODO: perform in background job
                let outcome = data.operation.perform(data.feature_collection.clone());
                let source_crs = match data.source_crs.clone() {
                    Some(crs) => crs,
                    None => {
                        bevy::log::error!("Source CRS is not set for the operation");
                        return None;
                    }
                };
                let action = match outcome {
                    Ok(rgis_geo_ops::Outcome::FeatureCollection(feature_collection)) => {
                        Some(OperationAction::CreateLayer {
                            feature_collection,
                            name: "FOOOOO".into(), // FIXME
                            source_crs,
                        })
                    }
                    Ok(rgis_geo_ops::Outcome::Text(text)) => {
                        Some(OperationAction::RenderMessage(text))
                    }
                    Err(e) => {
                        bevy::log::error!(
                            "Encountered an error during the operation: {}",
                            e
                        );
                        None
                    }
                };
                *self.state = None;
                action
            }
            rgis_geo_ops::Action::RenderUi => {
                let mut is_open = true;
                egui::Window::new("Operation")
                    .open(&mut is_open)
                    .show(self.egui_ctx, |ui| {
                        data.operation.ui(ui, &data.feature_collection);
                    });

                if !is_open {
                    *self.state = None;
                }
                None
            }
        }
    }
}
