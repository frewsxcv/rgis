pub struct ProjJsTransformer {
    source_crs: String,
    target_crs: String,
}

impl crate::Transformer for ProjJsTransformer {
    fn setup(source_crs: &str, target_crs: &str) -> Self {
        ProjJsTransformer {
            source_crs: source_crs.to_owned(),
            target_crs: target_crs.to_owned(),
        }
    }

    fn transform(&self, geometry: &mut geo::Geometry) -> Result<(), crate::TransformError> {
        geo_proj_js::transform(geometry, &self.source_crs, &self.target_crs)?;
        Ok(())
    }
}
