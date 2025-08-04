use crate::{Operation, OperationEntry, Outcome};
use geo_projected::UnprojectedScalar;
use std::{error, mem};

#[derive(Default)]
pub struct ConvexHull {
    geometries: Vec<geo::Geometry<UnprojectedScalar>>,
}

impl OperationEntry for ConvexHull {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::all();
    const NAME: &'static str = "Convex hull";

    type Op = ConvexHull;
}

impl Operation for ConvexHull {
    fn visit_geometry(&mut self, geometry: &geo::Geometry<UnprojectedScalar>) {
        self.geometries.push(geometry.clone());
    }

    fn finalize(&mut self) -> Result<Outcome, Box<dyn error::Error>> {
        use geo::ConvexHull;

        let geometries = mem::take(&mut self.geometries);
        let outcome = geo::GeometryCollection(geometries).convex_hull();

        Ok(Outcome::FeatureCollection(
            geo_features::FeatureCollection::from_geometry(outcome.into()),
        ))
    }
}
