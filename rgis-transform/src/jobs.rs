use crate::Transformer;

pub struct ReprojectGeometryJob {
    pub feature_collection: geo_projected::Unprojected<geo_features::FeatureCollection>,
    pub layer_id: rgis_layer_id::LayerId,
    pub source_crs: String,
    pub target_crs: String,
}

pub struct ReprojectGeometryJobOutcome {
    pub feature_collection: geo_projected::Projected<geo_features::FeatureCollection>,
    pub layer_id: rgis_layer_id::LayerId,
    pub target_crs: String,
}

impl bevy_jobs::Job for ReprojectGeometryJob {
    type Outcome = Result<ReprojectGeometryJobOutcome, crate::TransformError>;

    fn name(&self) -> String {
        "Projecting layer".to_string()
    }

    fn perform(
        mut self,
        progress_sender: bevy_jobs::Context,
    ) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            let total = self.feature_collection.features_iter_mut().count();

            let transformer = crate::DefaultTransformer::setup(&self.source_crs, &self.target_crs);

            for (i, feature) in self.feature_collection.features_iter_mut().enumerate() {
                let _ = progress_sender.send_progress((100 * i / total) as u8).await;

                if let Some(ref mut geometry) = &mut feature.0.geometry {
                    transformer.transform(geometry)?;
                }

                feature.0.recalculate_bounding_rect()?;
            }

            self.feature_collection.0.recalculate_bounding_rect()?;

            Ok(ReprojectGeometryJobOutcome {
                feature_collection: self.feature_collection.into_projected(),
                layer_id: self.layer_id,
                target_crs: self.target_crs,
            })
        })
    }
}
