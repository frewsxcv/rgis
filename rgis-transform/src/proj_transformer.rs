use geo::Transform;

pub struct ProjTransformer {
    proj_transformer: proj::Proj,
}

impl crate::Transformer for ProjTransformer {
    fn setup(source_crs: &str, target_crs: &str) -> Self {
        ProjTransformer {
            proj_transformer: proj::Proj::new_known_crs(source_crs, target_crs, None).unwrap(),
        }
    }

    fn transform(&self, geometry: &mut geo::Geometry) -> Result<(), crate::TransformError> {
        geometry.transform(&self.proj_transformer).unwrap();
        Ok(())
    }
}
