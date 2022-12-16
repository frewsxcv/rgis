pub struct ReprojectGeometryTask {
    pub feature_collection: geo_projected::Unprojected<geo_features::FeatureCollection>,
    pub layer_id: rgis_layer_id::LayerId,
    pub source_crs: String,
    pub target_crs: String,
}

pub struct ReprojectGeometryTaskOutcome {
    pub feature_collection: geo_projected::Projected<geo_features::FeatureCollection>,
    pub layer_id: rgis_layer_id::LayerId,
    pub target_crs: String,
}

impl bevy_jobs::Job for ReprojectGeometryTask {
    type Outcome = Result<ReprojectGeometryTaskOutcome, crate::TransformError>;

    fn name(&self) -> String {
        "Projecting layer".to_string()
    }

    fn perform(mut self) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            for feature in self.feature_collection.features_iter_mut() {
                if let Some(ref mut geometry) = &mut feature.0.geometry {
                    crate::transform(geometry, &self.source_crs, &self.target_crs)?;
                }

                feature.0.recalculate_bounding_rect()?;
            }

            self.feature_collection.0.recalculate_bounding_rect()?;

            Ok(ReprojectGeometryTaskOutcome {
                feature_collection: self.feature_collection.into_projected(),
                layer_id: self.layer_id,
                target_crs: self.target_crs,
            })
        })
    }
}
