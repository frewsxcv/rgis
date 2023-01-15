use crate::{Operation, OperationEntry, Outcome};
use geo::CoordsIter;
use geo::Simplify as GeoSimplify;
use std::{error, mem};

#[derive(Default)]
pub struct Simplify {
    simplified: geo::GeometryCollection,
    epsilon_text: String,
    epsilon: Option<f64>,
    execute_pressed: bool,
}

impl OperationEntry for Simplify {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::from_bits_truncate(
        geo_geom_type::GeomType::LINE_STRING.bits()
            | geo_geom_type::GeomType::MULTI_LINE_STRING.bits()
            | geo_geom_type::GeomType::POLYGON.bits()
            | geo_geom_type::GeomType::MULTI_POLYGON.bits(),
    );
    const NAME: &'static str = "Simplify geometries";

    fn build() -> Box<dyn Operation + Send + Sync> {
        Box::<Simplify>::default()
    }
}

impl Operation for Simplify {
    fn next_action(&self) -> crate::Action {
        if self.execute_pressed {
            crate::Action::Perform
        } else {
            crate::Action::RenderUi
        }
    }

    fn ui(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        feature_collection: &geo_projected::Unprojected<geo_features::FeatureCollection>,
    ) {
        ui.label("Epsilon:");
        ui.text_edit_singleline(&mut self.epsilon_text);
        let button = bevy_egui::egui::Button::new("Execute");
        match self.epsilon_text.parse::<f64>() {
            Ok(f) => {
                self.epsilon = Some(f);
                ui.label(format!(
                    "Previous # of nodes: {}",
                    feature_collection.0.coords_count()
                ));
                let feature_collection = match self.perform(feature_collection.clone()) {
                    // TODO: CLONE ABOVE
                    Ok(Outcome::FeatureCollection(fc)) => fc,
                    _ => {
                        ui.label("<ENCOUNTERED AN ERROR>");
                        return;
                    }
                };
                ui.label(format!(
                    "Simplified # of nodes: {}",
                    feature_collection.0.coords_count()
                ));
                if ui.add_enabled(true, button).clicked() {
                    self.execute_pressed = true;
                }
            }
            Err(_) => {
                ui.add_enabled(false, button);
            }
        };
    }

    fn visit_line_string(&mut self, line_string: geo::LineString) {
        self.simplified
            .0
            .push(line_string.simplify(&self.epsilon.unwrap()).into());
    }

    fn visit_multi_line_string(&mut self, multi_line_string: geo::MultiLineString) {
        self.simplified
            .0
            .push(multi_line_string.simplify(&self.epsilon.unwrap()).into());
    }

    fn visit_polygon(&mut self, polygon: geo::Polygon) {
        let simplified = polygon.simplify(&self.epsilon.unwrap());
        debug_assert!(simplified.exterior().0.len() >= 4);
        for interior in polygon.interiors() {
            debug_assert!(interior.0.len() >= 4);
        }
        self.simplified.0.push(simplified.into());
    }

    fn visit_multi_polygon(&mut self, multi_polygon: geo::MultiPolygon) {
        self.simplified
            .0
            .push(multi_polygon.simplify(&self.epsilon.unwrap()).into());
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
