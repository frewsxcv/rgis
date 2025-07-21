use geo_projected::CastTo;

pub struct ReprojectGeometryJob {
    pub feature_collection: geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
    pub layer_id: rgis_layer_id::LayerId,
    pub source_epsg_code: u16,
    pub target_epsg_code: u16,
}

pub struct ReprojectGeometryJobOutcome {
    pub feature_collection: geo_features::FeatureCollection<geo_projected::ProjectedScalar>,
    pub layer_id: rgis_layer_id::LayerId,
    pub target_crs_epsg_code: u16,
}

impl bevy_jobs::Job for ReprojectGeometryJob {
    type Outcome = Result<ReprojectGeometryJobOutcome, geo_geodesy::Error>;

    fn name(&self) -> String {
        "Projecting layer".to_string()
    }

    async fn perform(self, progress_sender: bevy_jobs::Context) -> Self::Outcome {
        let total = self.feature_collection.features.len();

        let transformer =
            geo_geodesy::Transformer::setup(self.source_epsg_code, self.target_epsg_code)?;

        let mut feature_collection = self.feature_collection.cast::<geo_projected::Projected>();

        for (i, feature) in feature_collection.features.iter_mut().enumerate() {
            let _ = progress_sender.send_progress((100 * i / total) as u8).await;

            if let Some(ref mut geometry) = &mut feature.geometry {
                transformer.transform(geometry)?;
            }

            feature.recalculate_bounding_rect();
        }

        feature_collection.recalculate_bounding_rect();

        Ok(ReprojectGeometryJobOutcome {
            feature_collection,
            layer_id: self.layer_id,
            target_crs_epsg_code: self.target_epsg_code,
        })
    }
}
