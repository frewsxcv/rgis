use crate::{Operation, OperationEntry, Outcome};
use geo::TriangulateEarcut;
use std::error;

impl OperationEntry for Triangulate {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::from_bits_truncate(
        geo_geom_type::GeomType::POLYGON.bits() | geo_geom_type::GeomType::MULTI_POLYGON.bits(),
    );
    const NAME: &'static str = "Triangulate";

    fn build() -> Box<dyn Operation + Send + Sync> {
        Box::<Triangulate>::default()
    }
}

#[derive(Default)]
pub struct Triangulate {
    triangles: Vec<geo::Triangle>,
}

impl Operation for Triangulate {
    fn visit_polygon(&mut self, polygon: geo::Polygon) {
        self.triangles.extend(polygon.triangulate_earcut_iter());
    }

    fn visit_multi_polygon(&mut self, multi_polygon: geo::MultiPolygon) {
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

        Ok(Outcome::FeatureCollection(geo_projected::Unprojected::new(
            geo_features::FeatureCollection::from_geometry(multi_polygon.into())?,
        )))
    }
}
