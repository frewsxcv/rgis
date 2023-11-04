use geo::Transform;
use std::error;

pub struct ProjTransformer {
    proj_transformer: proj::Proj,
}

impl crate::Transformer for ProjTransformer {
    fn setup(
        source_crs: &str,
        target_crs: &str,
    ) -> Result<Self, Box<dyn error::Error + Send + Sync>> {
        Ok(ProjTransformer {
            proj_transformer: proj::Proj::new_known_crs(source_crs, target_crs, None)?,
        })
    }

    fn transform(&self, geometry: &mut geo::Geometry) -> Result<(), crate::TransformError> {
        geometry.transform(&self.proj_transformer).unwrap();
        Ok(())
    }
}
