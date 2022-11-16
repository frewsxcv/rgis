use crate::{Operation, OperationEntry, Outcome};
use geo::Simplify as GeoSimplify;
use std::{error, mem};

// TODO: This should be calculated dynamically
const EPSILON: f64 = 1.;

#[derive(Default)]
pub struct Simplify {
    simplified: geo::GeometryCollection,
}

impl OperationEntry for Simplify {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::from_bits_truncate(
        geo_geom_type::GeomType::LINE_STRING.bits()
            | geo_geom_type::GeomType::MULTI_LINE_STRING.bits()
            | geo_geom_type::GeomType::POLYGON.bits()
            | geo_geom_type::GeomType::MULTI_POLYGON.bits(),
    );
    const NAME: &'static str = "Simplify geometries";
    const HAS_GUI: bool = true;

    fn build() -> Box<dyn Operation> {
        Box::<Simplify>::default()
    }
}

impl Operation for Simplify {
    fn ui(&self, ui: &mut bevy_egui::egui::Ui) {
        let mut s = String::new();
        ui.text_edit_singleline(&mut s);
    }

    fn visit_line_string(&mut self, line_string: geo::LineString) {
        self.simplified
            .0
            .push(line_string.simplify(&EPSILON).into());
    }

    fn visit_multi_line_string(&mut self, polygon: geo::MultiLineString) {
        self.simplified.0.push(polygon.simplify(&EPSILON).into());
    }

    fn visit_polygon(&mut self, polygon: geo::Polygon) {
        self.simplified.0.push(polygon.simplify(&EPSILON).into());
    }

    fn visit_multi_polygon(&mut self, multi_polygon: geo::MultiPolygon) {
        self.simplified
            .0
            .push(multi_polygon.simplify(&EPSILON).into());
    }

    fn finalize(&mut self) -> Result<Outcome, Box<dyn error::Error>> {
        let simplified = mem::take(&mut self.simplified);
        Ok(Outcome::FeatureCollection(geo_projected::Unprojected::new(
            geo_features::FeatureCollection::from_geometry(geo::Geometry::GeometryCollection(
                simplified,
            ))?,
        )))
    }
}
