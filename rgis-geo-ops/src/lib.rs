#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

pub enum Outcome {
    Text,
    FeatureCollection(geo_features::FeatureCollection),
}

pub trait Operation: Sized {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType;

    fn name(&self) -> &'static str;

    fn perform(
        mut self,
        feature_collection: geo_features::FeatureCollection,
    ) -> Option<geo_features::FeatureCollection> {
        for feature in feature_collection.features {
            self.visit_feature(feature);
        }
        match self.finalize() {
            Outcome::FeatureCollection(feature_collection) => Some(feature_collection),
            // TODO: handle text outcome
            _ => todo!(),
        }
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
            geo::Geometry::GeometryCollection(g) => todo!(),
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

#[derive(Default)]
pub struct ConvexHull {
    geometries: Vec<geo::Geometry>,
}

impl Operation for ConvexHull {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::ALL;

    fn name(&self) -> &'static str {
        "Convex hull"
    }

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

#[derive(Default)]
pub struct Outliers {
    points: Vec<geo::Point>,
}

impl Operation for Outliers {
    const ALLOWED_GEOM_TYPES: geo_geom_type::GeomType = geo_geom_type::GeomType::from_bits_truncate(
        geo_geom_type::GeomType::POINT.bits() | geo_geom_type::GeomType::MULTI_POINT.bits(),
    );

    fn name(&self) -> &'static str {
        "Detect outliers"
    }

    fn visit_point(&mut self, point: geo::Point) {
        self.points.push(point);
    }

    fn visit_multi_point(&mut self, multi_point: geo::MultiPoint) {
        self.points.extend(multi_point.0.into_iter());
    }

    fn finalize(self) -> Outcome {
        use geo::OutlierDetection;

        let mut non_outliers = vec![];

        let multi_point = geo::MultiPoint(self.points);

        for (outlier_score, coord) in multi_point.outliers(15).iter().zip(multi_point.0.iter()) {
            if *outlier_score < 2. {
                non_outliers.push(*coord);
            }
        }

        let new_multi_point = geo::MultiPoint::new(non_outliers);

        Outcome::FeatureCollection(
            geo_features::FeatureCollection::from_geometry(new_multi_point.into()).unwrap(),
        )
    }
}
