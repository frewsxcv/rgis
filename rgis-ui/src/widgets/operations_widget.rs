use crate::panels::side_panel::{Events, OperationButton};
use bevy_egui::egui::{self, Align, Layout};

pub struct OperationsWidget<'a, 'w> {
    pub layer: &'a rgis_layers::Layer,
    pub events: &'a mut Events<'w>,
}

impl egui::Widget for OperationsWidget<'_, '_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
            if ui.button("Bounding rect").clicked() {
                if let Ok(bounding_rect) = self.layer.unprojected_feature_collection.bounding_rect()
                {
                    let feature_collection =
                        geo_features::FeatureCollection::from_geometry(bounding_rect.into());
                    self.events
                        .create_layer_event_writer
                        .write(rgis_events::CreateLayerEvent {
                            feature_collection,
                            name: "Bounding rect".into(), // FIXME
                            source_crs: self.layer.crs,
                        });
                }
            }

            ui.add(OperationButton::<rgis_geo_ops::ConvexHull>::new(
                self.events,
                self.layer,
            ));
            ui.add(OperationButton::<rgis_geo_ops::Outliers>::new(
                self.events,
                self.layer,
            ));
            ui.add(OperationButton::<rgis_geo_ops::Rotate>::new(
                self.events,
                self.layer,
            ));
            ui.add(OperationButton::<rgis_geo_ops::Simplify>::new(
                self.events,
                self.layer,
            ));
            ui.add(OperationButton::<rgis_geo_ops::Smoothing>::new(
                self.events,
                self.layer,
            ));
            ui.add(OperationButton::<rgis_geo_ops::Triangulate>::new(
                self.events,
                self.layer,
            ));
            ui.add(OperationButton::<rgis_geo_ops::UnsignedArea>::new(
                self.events,
                self.layer,
            ));
        })
        .response
    }
}
