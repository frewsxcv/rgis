use crate::panels::side::{Events, OperationButton};
use bevy_egui::egui::{self, Align, Layout};
use rgis_layer_messages::CreateLayerMessage;

pub struct Operations<'a, 'w> {
    pub layer_id: rgis_primitives::LayerId,
    pub geom_type: Option<geo_geom_type::GeomType>,
    pub crs: &'a rgis_layers::LayerCrs,
    pub unprojected_fc: Option<&'a geo_features::FeatureCollection<geo_projected::UnprojectedScalar>>,
    pub events: &'a mut Events<'w>,
}

impl egui::Widget for Operations<'_, '_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
            if ui.button("Bounding rect").clicked() {
                if let Some(fc) = self.unprojected_fc {
                    if let Ok(bounding_rect) = fc.bounding_rect() {
                        let feature_collection =
                            geo_features::FeatureCollection::from_geometry(bounding_rect.into());
                        self.events
                            .create_layer_event_writer
                            .write(CreateLayerMessage {
                                feature_collection: std::sync::Arc::new(feature_collection),
                                name: "Bounding rect".into(), // FIXME
                                source_crs: self.crs.0.clone(),
                            });
                    }
                }
            }

            ui.add(OperationButton::<rgis_geo_ops::ConvexHull>::new(
                self.events,
                self.layer_id,
                self.geom_type,
            ));
            ui.add(OperationButton::<rgis_geo_ops::Outliers>::new(
                self.events,
                self.layer_id,
                self.geom_type,
            ));
            ui.add(OperationButton::<rgis_geo_ops::Rotate>::new(
                self.events,
                self.layer_id,
                self.geom_type,
            ));
            ui.add(OperationButton::<rgis_geo_ops::Simplify>::new(
                self.events,
                self.layer_id,
                self.geom_type,
            ));
            ui.add(OperationButton::<rgis_geo_ops::Smoothing>::new(
                self.events,
                self.layer_id,
                self.geom_type,
            ));
            ui.add(OperationButton::<rgis_geo_ops::Triangulate>::new(
                self.events,
                self.layer_id,
                self.geom_type,
            ));
            ui.add(OperationButton::<rgis_geo_ops::UnsignedArea>::new(
                self.events,
                self.layer_id,
                self.geom_type,
            ));
        })
        .response
    }
}
