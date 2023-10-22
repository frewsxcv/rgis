use crate::{Operation, OperationEntry, Outcome};
use geo::{GeometryCollection, Rotate as GeoRotate};
use std::mem;

#[derive(Default)]
pub struct Rotate {
    rotated: GeometryCollection,
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
        feature_collection: geo_projected::Unprojected<geo_features::FeatureCollection>,
    ) {
        self.rotated = feature_collection.0.to_geometry_collection();
        self.rotated.rotate_around_centroid_mut(45.);
    }

    fn finalize(&mut self) -> Result<crate::Outcome, Box<dyn std::error::Error>> {
        let gc = mem::take(&mut self.rotated);
        Ok(Outcome::FeatureCollection(geo_projected::Unprojected::new(
            geo_features::FeatureCollection::from_geometry(geo::Geometry::GeometryCollection(gc)),
        )))
    }
}
