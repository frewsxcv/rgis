#[derive(thiserror::Error, Debug)]
pub enum TransformError {
    #[cfg(target_arch = "wasm32")]
    #[error("{0}")]
    GeoProjJs(#[from] geo_proj_js::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("{0}")]
    Proj(#[from] geo::algorithm::proj::TransformError),
    #[error("{0}")]
    BoundingRect(#[from] geo_features::BoundingRectError),
}

pub struct ReprojectGeometryTask {
    pub feature_collection: geo_features::FeatureCollection,
    pub layer_id: rgis_layer_id::LayerId,
    pub source_crs: String,
    pub target_crs: String,
}

pub struct ReprojectGeometryTaskOutcome {
    pub feature_collection: geo_features::FeatureCollection,
    pub layer_id: rgis_layer_id::LayerId,
    pub target_crs: String,
}

impl bevy_jobs::Job for ReprojectGeometryTask {
    type Outcome = Result<ReprojectGeometryTaskOutcome, TransformError>;

    fn name(&self) -> String {
        "Projecting layer".to_string()
    }

    fn perform(mut self) -> bevy_jobs::PerformReturn<Self::Outcome> {
        Box::pin(async move {
            for feature in self.feature_collection.features.iter_mut() {
                #[cfg(target_arch = "wasm32")]
                {
                    geo_proj_js::transform(
                        &mut feature.geometry,
                        &self.source_crs,
                        &self.target_crs,
                    )?;
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    use geo::transform::Transform;
                    feature
                        .geometry
                        .transform_crs_to_crs(&self.source_crs, &self.target_crs)?;
                }

                feature.recalculate_bounding_rect()?;
            }

            self.feature_collection.recalculate_bounding_rect()?;

            Ok(ReprojectGeometryTaskOutcome {
                feature_collection: self.feature_collection,
                layer_id: self.layer_id,
                target_crs: self.target_crs,
            })
        })
    }
}
