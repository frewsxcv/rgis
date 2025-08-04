use bevy::prelude::Event;
use rgis_primitives::LayerId;

#[derive(Event)]
pub struct OpenOperationWindowEvent {
    pub operation: Box<dyn Send + Sync + rgis_geo_ops::Operation>,
    pub feature_collection: geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
}

#[derive(Event)]
pub struct PerformOperationEvent {
    pub operation: Box<dyn Send + Sync + rgis_geo_ops::Operation>,
    pub layer_id: LayerId,
}
