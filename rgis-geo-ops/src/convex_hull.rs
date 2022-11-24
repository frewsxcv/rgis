use crate::{Operation, OperationEntry, Outcome};
use std::{error, mem};

#[derive(Default)]
pub struct ConvexHull {
    geometries: Vec<geo::Geometry>,
}

impl OperationEntry for ConvexHull {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::all();
    const NAME: &'static str = "Convex hull";

    fn build() -> Box<dyn Operation + Send + Sync> {
        Box::<ConvexHull>::default()
    }
}

impl Operation for ConvexHull {
    fn visit_geometry(&mut self, geometry: geo::Geometry) {
        self.geometries.push(geometry);
    }

    fn finalize(&mut self) -> Result<Outcome, Box<dyn error::Error>> {
        use geo::ConvexHull;

        let geometries = mem::take(&mut self.geometries);
        let outcome = geo::GeometryCollection(geometries).convex_hull();

        Ok(Outcome::FeatureCollection(geo_projected::Unprojected::new(
            geo_features::FeatureCollection::from_geometry(outcome.into())?,
        )))
    }
}
