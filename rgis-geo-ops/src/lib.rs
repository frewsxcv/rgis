#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

pub enum Outcome {
    Text,
    Feature(geo_features::Feature),
}

pub trait Operation {
    fn name(&self) -> &'static str;

    fn perform(&self, feature_collection: geo_features::FeatureCollection) -> Option<Outcome> {
        for feature in feature_collection.features {
            self.perform_feature(feature);
        }
        todo!()
    }

    fn perform_feature(&self, feature: geo_features::Feature) -> Option<Outcome> {
        match feature.geometry {
            Some(g) => self.perform_geometry(g),
            None => None,
        }
    }

    fn is_geometry_allowed(&self, geometry: geo::Geometry) -> bool;

    fn perform_geometry(&self, geometry: geo::Geometry) -> Option<Outcome> {
        match geometry {
            geo::Geometry::Point(g) => self.perform_point(g),
            geo::Geometry::Line(_) => todo!(),
            geo::Geometry::LineString(_) => todo!(),
            geo::Geometry::Polygon(_) => todo!(),
            geo::Geometry::MultiPoint(g) => self.perform_multi_point(g),
            geo::Geometry::MultiLineString(_) => todo!(),
            geo::Geometry::MultiPolygon(_) => todo!(),
            geo::Geometry::Rect(_) => todo!(),
            geo::Geometry::Triangle(_) => todo!(),
            geo::Geometry::GeometryCollection(_) => todo!(),
        }
    }

    fn perform_point(&self, point: geo::Point) -> Option<Outcome> {
        None
    }

    fn perform_multi_point(&self, point: geo::MultiPoint) -> Option<Outcome> {
        None
    }
}

pub struct ConvexHull;

impl Operation for ConvexHull {
    fn name(&self) -> &'static str {
        "Convex hull"
    }

    fn is_geometry_allowed(&self, _geometry: geo::Geometry) -> bool {
        true
    }

    fn perform_geometry(&self, geometry: geo::Geometry) -> Option<Outcome> {
        use geo::ConvexHull;

        Some(Outcome::Feature(
            geo_features::FeatureBuilder::new()
                .with_geometry(geometry.convex_hull().into())
                .build()
                .unwrap(),
        ))
    }
}

pub struct Outliers;

impl Operation for Outliers {
    fn name(&self) -> &'static str {
        "Detect outliers"
    }

    fn is_geometry_allowed(&self, geometry: geo::Geometry) -> bool {
        matches!(geometry, geo::Geometry::MultiPoint(_))
    }

    fn perform_multi_point(&self, geometry: geo::MultiPoint) -> Option<Outcome> {
        // use geo::Outliers;

        todo!()
    }
}
