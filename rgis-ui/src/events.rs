pub struct OpenOperationWindowEvent {
    pub operation: Box<dyn Send + Sync + rgis_geo_ops::Operation>,
    pub feature_collection: geo_projected::Unprojected<geo_features::FeatureCollection>,
}
