use geo_projected::UnprojectedScalar;
use std::error;

mod unsigned_area;
pub use unsigned_area::UnsignedArea;

mod convex_hull;
pub use convex_hull::ConvexHull;

mod outliers;
pub use outliers::Outliers;

mod rotate;
pub use rotate::Rotate;

mod simplify;
pub use simplify::Simplify;

mod smoothing;
pub use smoothing::Smoothing;

mod triangulate;
pub use triangulate::Triangulate;

pub enum Outcome {
    Text(String),
    FeatureCollection(geo_features::FeatureCollection<UnprojectedScalar>),
}

pub trait OperationEntry {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType;
    const NAME: &'static str;

    fn build() -> Box<dyn Operation + Send + Sync>;
}

pub enum Action {
    RenderUi,
    Perform,
}

pub trait Operation {
    fn perform(
        &mut self,
        feature_collection: geo_features::FeatureCollection<UnprojectedScalar>,
    ) -> Result<Outcome, Box<dyn error::Error>> {
        self.visit_feature_collection(&feature_collection);
        for feature in feature_collection.features.into_iter() {
            self.visit_feature(&feature);
            if let Some(geometry) = feature.geometry {
                self.visit_geometry(&geometry);
                match geometry {
                    geo::Geometry::Point(g) => self.visit_point(&g),
                    geo::Geometry::Line(g) => self.visit_line(&g),
                    geo::Geometry::LineString(g) => self.visit_line_string(&g),
                    geo::Geometry::Polygon(g) => self.visit_polygon(&g),
                    geo::Geometry::MultiPoint(g) => self.visit_multi_point(&g),
                    geo::Geometry::MultiLineString(g) => self.visit_multi_line_string(&g),
                    geo::Geometry::MultiPolygon(g) => self.visit_multi_polygon(&g),
                    geo::Geometry::Rect(g) => self.visit_rect(&g),
                    geo::Geometry::Triangle(g) => self.visit_triangle(&g),
                    geo::Geometry::GeometryCollection(geometry_collection) => {
                        for geometry in geometry_collection {
                            self.visit_geometry(&geometry);
                        }
                    }
                }
            }
        }
        self.finalize()
    }

    fn finalize(&mut self) -> Result<Outcome, Box<dyn error::Error>>;

    fn next_action(&self) -> Action {
        Action::Perform
    }

    fn ui(
        &mut self,
        _ui: &mut bevy_egui::egui::Ui,
        _feature_collection: &geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
    ) {
    }

    fn visit_feature_collection(
        &mut self,
        _feature_collection: &geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
    ) {
    }

    fn visit_feature(
        &mut self,
        _feature: &geo_features::Feature<geo_projected::UnprojectedScalar>,
    ) {
    }

    fn visit_geometry(&mut self, _geometry: &geo::Geometry<geo_projected::UnprojectedScalar>) {}

    fn visit_point(&mut self, _point: &geo::Point<geo_projected::UnprojectedScalar>) {}

    fn visit_line(&mut self, _line: &geo::Line<geo_projected::UnprojectedScalar>) {}

    fn visit_line_string(
        &mut self,
        _line_string: &geo::LineString<geo_projected::UnprojectedScalar>,
    ) {
    }

    fn visit_polygon(&mut self, _polygon: &geo::Polygon<geo_projected::UnprojectedScalar>) {}

    fn visit_multi_point(
        &mut self,
        _multi_point: &geo::MultiPoint<geo_projected::UnprojectedScalar>,
    ) {
    }

    fn visit_multi_line_string(
        &mut self,
        _multi_line_string: &geo::MultiLineString<geo_projected::UnprojectedScalar>,
    ) {
    }

    fn visit_multi_polygon(
        &mut self,
        _multi_polygon: &geo::MultiPolygon<geo_projected::UnprojectedScalar>,
    ) {
    }

    fn visit_rect(&mut self, _rect: &geo::Rect<geo_projected::UnprojectedScalar>) {}

    fn visit_triangle(&mut self, _triangle: &geo::Triangle<geo_projected::UnprojectedScalar>) {}
}
