#[derive(thiserror::Error, Debug)]
pub enum TransformError {
    #[cfg(target_arch = "wasm32")]
    #[error("{0}")]
    GeoProjJs(#[from] geo_proj_js::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("{0}")]
    Proj(#[from] proj::TransformError),
}

pub struct ReprojectGeometryTask {
    pub geometry: geo::Geometry<f64>,
    pub layer_id: rgis_layer_id::LayerId,
    pub source_crs: String,
    pub target_crs: String,
}

pub struct ReprojectGeometryTaskOutcome {
    pub geometry: geo::Geometry<f64>,
    pub layer_id: rgis_layer_id::LayerId,
    pub target_crs: String,
}

impl rgis_task::Task for ReprojectGeometryTask {
    type Outcome = Result<ReprojectGeometryTaskOutcome, TransformError>;

    fn name(&self) -> String {
        "Projecting layer".to_string()
    }

    fn perform(mut self) -> rgis_task::PerformReturn<Self::Outcome> {
        Box::pin(async move {
            #[cfg(target_arch = "wasm32")]
            {
                geo_proj_js::transform(&mut self.geometry, &self.source_crs, &self.target_crs)?;
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                use geo::transform::Transform;
                self.geometry.transform_crs_to_crs(&self.source_crs, &self.target_crs)?;
            }

            Ok(ReprojectGeometryTaskOutcome {
                geometry: self.geometry,
                layer_id: self.layer_id,
                target_crs: self.target_crs,
            })
        })
    }
}
