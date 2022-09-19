use crate::{Operation, Outcome};

#[derive(Default)]
pub struct ConvexHull {
    geometries: Vec<geo::Geometry>,
}

impl Operation for ConvexHull {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::all();

    const NAME: &'static str = "Convex hull";

    fn visit_geometry(&mut self, geometry: geo::Geometry) {
        self.geometries.push(geometry);
    }

    fn finalize(self) -> Outcome {
        use geo::ConvexHull;

        let outcome = geo::GeometryCollection(self.geometries).convex_hull();

        Outcome::FeatureCollection(
            geo_features::FeatureCollection::from_geometry(outcome.into()).unwrap(),
        )
    }
}
