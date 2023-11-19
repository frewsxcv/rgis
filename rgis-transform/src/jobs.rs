pub struct ReprojectGeometryJob {
    pub feature_collection: geo_projected::Unprojected<geo_features::FeatureCollection>,
    pub layer_id: rgis_layer_id::LayerId,
    pub source_epsg_code: u16,
    pub target_epsg_code: u16,
}

pub struct ReprojectGeometryJobOutcome {
    pub feature_collection: geo_projected::Projected<geo_features::FeatureCollection>,
    pub layer_id: rgis_layer_id::LayerId,
    pub target_crs_epsg_code: u16,
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

            let transformer =
                crate::ProjTransformer::setup(self.source_epsg_code, self.target_epsg_code)
                    .map_err(crate::TransformError::SetupError)?;

            for (i, feature) in self.feature_collection.features_iter_mut().enumerate() {
                let _ = progress_sender.send_progress((100 * i / total) as u8).await;

                if let Some(ref mut geometry) = &mut feature.0.geometry {
                    transformer.transform(geometry)?;
                }

                feature.0.recalculate_bounding_rect();
            }

            self.feature_collection.0.recalculate_bounding_rect();

            Ok(ReprojectGeometryJobOutcome {
                feature_collection: self.feature_collection.into_projected(),
                layer_id: self.layer_id,
                target_crs_epsg_code: self.target_epsg_code,
            })
        })
    }
}
