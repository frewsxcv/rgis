use crate::{Operation, OperationEntry, Outcome};
use geo::TriangulateEarcut;
use geo_projected::UnprojectedScalar;
use std::error;

impl OperationEntry for Triangulate {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::from_bits_truncate(
        geo_geom_type::GeomType::POLYGON.bits() | geo_geom_type::GeomType::MULTI_POLYGON.bits(),
    );
    const NAME: &'static str = "Triangulate";

    type Op = Triangulate;
}

#[derive(Default)]
pub struct Triangulate {
    triangles: Vec<geo::Triangle<UnprojectedScalar>>,
}

impl Operation for Triangulate {
    fn visit_polygon(&mut self, polygon: &geo::Polygon<UnprojectedScalar>) {
        self.triangles.extend(polygon.earcut_triangles_iter());
    }

    fn visit_multi_polygon(&mut self, multi_polygon: &geo::MultiPolygon<UnprojectedScalar>) {
        for polygon in multi_polygon {
            self.visit_polygon(polygon);
        }
    }

    fn finalize(&mut self) -> Result<Outcome, Box<dyn error::Error>> {
        let multi_polygon = geo::MultiPolygon(
            self.triangles
                .drain(..)
                .map(|triangle| triangle.to_polygon())
                .collect::<Vec<_>>(),
        );

        Ok(Outcome::FeatureCollection(
            geo_features::FeatureCollection::from_geometry(geo::Geometry::MultiPolygon(
                multi_polygon,
            )),
        ))
    }
}
