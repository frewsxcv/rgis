use crate::{Operation, OperationEntry, Outcome};
use geo::Area;
use std::error;

// FIXME: should this operate on the projected featurecollection instead of the unprojected?
#[derive(Default)]
pub struct UnsignedArea {
    total_area: f64,
}

impl OperationEntry for UnsignedArea {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::from_bits_truncate(
        geo_geom_type::GeomType::POLYGON.bits()
            | geo_geom_type::GeomType::MULTI_POLYGON.bits()
            | geo_geom_type::GeomType::RECT.bits()
            | geo_geom_type::GeomType::TRIANGLE.bits(),
    );
    const NAME: &'static str = "Area (unsigned)";

    fn build() -> Box<dyn Operation + Send + Sync> {
        Box::<UnsignedArea>::default()
    }
}

impl Operation for UnsignedArea {
    fn visit_polygon(&mut self, polygon: geo::Polygon) {
        self.total_area += polygon.unsigned_area();
    }

    fn visit_multi_polygon(&mut self, multi_polygon: geo::MultiPolygon) {
        for polygon in multi_polygon {
            self.total_area += polygon.unsigned_area();
        }
    }

    fn visit_triangle(&mut self, triangle: geo::Triangle) {
        self.total_area += triangle.unsigned_area();
    }

    fn visit_rect(&mut self, rect: geo::Rect) {
        self.total_area += rect.unsigned_area();
    }

    fn finalize(&mut self) -> Result<Outcome, Box<dyn error::Error>> {
        Ok(Outcome::Text(format!("Area: {}", self.total_area)))
    }
}
