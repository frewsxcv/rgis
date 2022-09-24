use crate::{Operation, Outcome};
use geo::Area;

// FIXME: should this operate on the projected featurecollection instead of the unprojected?
#[derive(Default)]
pub struct UnsignedArea {
    total_area: f64,
}

impl Operation for UnsignedArea {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::from_bits_truncate(
        geo_geom_type::GeomType::POLYGON.bits()
            | geo_geom_type::GeomType::MULTI_POLYGON.bits()
            | geo_geom_type::GeomType::RECT.bits()
            | geo_geom_type::GeomType::TRIANGLE.bits(),
    );
    const NAME: &'static str = "Area (unsigned)";
    type Error = !;

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

    fn finalize(self) -> Result<Outcome, Self::Error> {
        Ok(Outcome::Text(format!("Area: {}", self.total_area)))
    }
}
