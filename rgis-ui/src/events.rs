use bevy::prelude::Event;

#[derive(Event)]
pub struct OpenOperationWindowEvent {
    pub operation: Box<dyn Send + Sync + rgis_geo_ops::Operation>,
    pub feature_collection: geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
}
