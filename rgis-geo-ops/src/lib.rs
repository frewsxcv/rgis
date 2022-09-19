#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

mod unsigned_area;
pub use unsigned_area::UnsignedArea;

mod convex_hull;
pub use convex_hull::ConvexHull;

mod outliers;
pub use outliers::Outliers;

pub enum Outcome {
    Text(String),
    FeatureCollection(geo_features::FeatureCollection),
}

pub trait Operation: Sized {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType;
    const NAME: &'static str;

    fn perform(mut self, feature_collection: geo_features::FeatureCollection) -> Outcome {
        for feature in feature_collection.features {
            self.visit_feature(feature);
        }
        self.finalize()
    }

    fn finalize(self) -> Outcome;

    fn visit_feature(&mut self, feature: geo_features::Feature) {
        match feature.geometry {
            Some(g) => self.visit_geometry(g),
            None => (),
        }
    }

    fn visit_geometry(&mut self, geometry: geo::Geometry) {
        match geometry {
            geo::Geometry::Point(g) => self.visit_point(g),
            geo::Geometry::Line(g) => self.visit_line(g),
            geo::Geometry::LineString(g) => self.visit_line_string(g),
            geo::Geometry::Polygon(g) => self.visit_polygon(g),
            geo::Geometry::MultiPoint(g) => self.visit_multi_point(g),
            geo::Geometry::MultiLineString(g) => self.visit_multi_line_string(g),
            geo::Geometry::MultiPolygon(g) => self.visit_multi_polygon(g),
            geo::Geometry::Rect(g) => self.visit_rect(g),
            geo::Geometry::Triangle(g) => self.visit_triangle(g),
            geo::Geometry::GeometryCollection(geometry_collection) => {
                for geometry in geometry_collection {
                    self.visit_geometry(geometry);
                }
            },
        }
    }

    fn visit_point(&mut self, _point: geo::Point) {}

    fn visit_line(&mut self, _line: geo::Line) {}

    fn visit_line_string(&mut self, _line_string: geo::LineString) {}

    fn visit_polygon(&mut self, _polygon: geo::Polygon) {}

    fn visit_multi_point(&mut self, _multi_point: geo::MultiPoint) {}

    fn visit_multi_line_string(&mut self, _multi_line_string: geo::MultiLineString) {}

    fn visit_multi_polygon(&mut self, _multi_polygon: geo::MultiPolygon) {}

    fn visit_rect(&mut self, _rect: geo::Rect) {}

    fn visit_triangle(&mut self, _triagnle: geo::Triangle) {}
}
