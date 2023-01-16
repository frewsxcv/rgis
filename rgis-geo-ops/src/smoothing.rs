use crate::{Operation, OperationEntry, Outcome};
use geo::{ChaikinSmoothing, GeometryCollection};
use std::mem;

#[derive(Default)]
pub struct Smoothing {
    smoothed: GeometryCollection,
}

impl OperationEntry for Smoothing {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::from_bits_truncate(
        geo_geom_type::GeomType::LINE_STRING.bits()
            | geo_geom_type::GeomType::MULTI_LINE_STRING.bits()
            | geo_geom_type::GeomType::POLYGON.bits()
            | geo_geom_type::GeomType::MULTI_POLYGON.bits(),
    );
    const NAME: &'static str = "Smooth geometries";

    fn build() -> Box<dyn Operation + Send + Sync> {
        Box::<Smoothing>::default()
    }
}

const NUM_ITERATIONS: usize = 2;

impl Operation for Smoothing {
    fn visit_line_string(&mut self, line_string: geo::LineString) {
        self.smoothed
            .0
            .push(line_string.chaikin_smoothing(NUM_ITERATIONS).into());
    }

    fn visit_multi_line_string(&mut self, multi_line_string: geo::MultiLineString) {
        self.smoothed
            .0
            .push(multi_line_string.chaikin_smoothing(NUM_ITERATIONS).into());
    }

    fn visit_polygon(&mut self, polygon: geo::Polygon) {
        self.smoothed
            .0
            .push(polygon.chaikin_smoothing(NUM_ITERATIONS).into());
    }

    fn visit_multi_polygon(&mut self, multi_polygon: geo::MultiPolygon) {
        self.smoothed
            .0
            .push(multi_polygon.chaikin_smoothing(NUM_ITERATIONS).into());
    }

    fn finalize(&mut self) -> Result<crate::Outcome, Box<dyn std::error::Error>> {
        let smoothed = mem::take(&mut self.smoothed);
        Ok(Outcome::FeatureCollection(geo_projected::Unprojected::new(
            geo_features::FeatureCollection::from_geometry(geo::Geometry::GeometryCollection(
                smoothed,
            ))?,
        )))
    }
}
