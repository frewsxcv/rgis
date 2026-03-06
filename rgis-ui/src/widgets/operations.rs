use crate::panels::side::{OperationButton, SidePanelAction};
use bevy_egui::egui::{self, Align, Layout};

pub struct Operations<'a> {
    pub layer: &'a rgis_layers::Layer,
}

impl Operations<'_> {
    pub fn show(self, ui: &mut egui::Ui, actions: &mut Vec<SidePanelAction>) {
        ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
            if ui.button("Bounding rect").clicked() {
                if let Some(fc) = self.layer.unprojected_feature_collection() {
                    if let Ok(bounding_rect) = fc.bounding_rect() {
                        let feature_collection =
                            geo_features::FeatureCollection::from_geometry(bounding_rect.into());
                        actions.push(SidePanelAction::CreateLayer {
                            feature_collection,
                            name: "Bounding rect".into(),
                            source_crs: self.layer.crs.clone(),
                        });
                    }
                }
            }

            add_operation_button::<rgis_geo_ops::ConvexHull>(ui, self.layer, actions);
            add_operation_button::<rgis_geo_ops::Outliers>(ui, self.layer, actions);
            add_operation_button::<rgis_geo_ops::Rotate>(ui, self.layer, actions);
            add_operation_button::<rgis_geo_ops::Simplify>(ui, self.layer, actions);
            add_operation_button::<rgis_geo_ops::Smoothing>(ui, self.layer, actions);
            add_operation_button::<rgis_geo_ops::Triangulate>(ui, self.layer, actions);
            add_operation_button::<rgis_geo_ops::UnsignedArea>(ui, self.layer, actions);
        });
    }
}

fn add_operation_button<Op: rgis_geo_ops::OperationEntry>(
    ui: &mut egui::Ui,
    layer: &rgis_layers::Layer,
    actions: &mut Vec<SidePanelAction>,
) {
    if ui.add(OperationButton::<Op>::new(layer)).clicked() {
        actions.push(SidePanelAction::PerformOperation {
            operation: Box::new(Op::build()),
            layer_id: layer.id,
        });
    }
}
