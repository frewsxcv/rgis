use crate::{Operation, OperationEntry, Outcome};
use geo::{GeometryCollection, Rotate as GeoRotate};
use geo_projected::UnprojectedScalar;
use std::mem;

#[derive(Default)]
pub struct Rotate {
    rotated: GeometryCollection<UnprojectedScalar>,
}

impl OperationEntry for Rotate {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::all();
    const NAME: &'static str = "Rotate geometries";

    fn build() -> Box<dyn Operation + Send + Sync> {
        Box::<Self>::default()
    }
}

impl Operation for Rotate {
    fn visit_feature_collection(
        &mut self,
        feature_collection: &geo_features::FeatureCollection<UnprojectedScalar>,
    ) {
        self.rotated = feature_collection.to_geometry_collection();
        self.rotated
            .rotate_around_centroid_mut(typed_num::TypedNum::new(45.))
    }

    fn finalize(&mut self) -> Result<crate::Outcome, Box<dyn std::error::Error>> {
        let gc = mem::take(&mut self.rotated);
        Ok(Outcome::FeatureCollection(
            geo_features::FeatureCollection::from_geometry(geo::Geometry::GeometryCollection(gc)),
        ))
    }
}
