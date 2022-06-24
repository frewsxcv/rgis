use geo::{BoundingRect, Contains};
use std::collections;

#[derive(Clone, Debug)]
pub struct Feature {
    pub geometry: geo::Geometry<f64>,
    // TODO: this should allow for non-string values
    pub properties: collections::HashMap<String, String>,
    pub bounding_rect: geo::Rect<f64>,
}

impl Feature {
    pub fn from_geometry(
        geometry: geo::Geometry<f64>,
        properties: collections::HashMap<String, String>,
    ) -> Result<Self, BoundingRectError> {
        let bounding_rect = geometry.bounding_rect().ok_or(BoundingRectError)?;

        Ok(Feature {
            geometry,
            properties,
            bounding_rect,
        })
    }

    pub fn recalculate_bounding_rect(&mut self) -> Result<(), BoundingRectError> {
        self.bounding_rect = self.geometry.bounding_rect().ok_or(BoundingRectError)?;
        Ok(())
    }
}

impl Contains<geo::Coordinate<f64>> for Feature {
    fn contains(&self, coord: &geo::Coordinate<f64>) -> bool {
        self.bounding_rect.contains(coord) && self.geometry.contains(coord)
    }
}

#[derive(Clone, Debug)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
    pub bounding_rect: geo::Rect<f64>,
}

#[derive(Debug)]
pub struct BoundingRectError;

impl FeatureCollection {
    pub fn from_feature(feature: Feature) -> Self {
        FeatureCollection {
            bounding_rect: feature.bounding_rect,
            features: vec![feature],
        }
    }

    pub fn from_features(features: Vec<Feature>) -> Self {
        FeatureCollection {
            bounding_rect: bounding_rect_from_features(&features),
            features,
        }
    }

    pub fn to_geometry_collection(&self) -> geo::GeometryCollection<f64> {
        geo::GeometryCollection(
            self.features
                .iter()
                .map(|f| f.geometry.clone())
                .collect::<Vec<_>>(),
        )
    }

    pub fn bounding_rect(&self) -> Result<geo::Rect<f64>, BoundingRectError> {
        // TODO: audit performance
        self.to_geometry_collection()
            .bounding_rect()
            .ok_or(BoundingRectError)
    }

    pub fn recalculate_bounding_rect(&mut self) -> Result<(), BoundingRectError> {
        self.bounding_rect = bounding_rect_from_features(&self.features);
        Ok(())
    }
}

fn bounding_rect_from_features(features: &[Feature]) -> geo::Rect<f64> {
    assert!(!features.is_empty());
    let mut bounding_rect = features[0].bounding_rect;
    for feature in &features[1..] {
        bounding_rect = rect_merge(bounding_rect, feature.bounding_rect);
    }
    bounding_rect
}

fn rect_merge<T: geo::CoordFloat>(a: geo::Rect<T>, b: geo::Rect<T>) -> geo::Rect<T> {
    geo::Rect::new(
        geo::Coordinate {
            x: a.min().x.min(b.min().x),
            y: a.min().y.min(b.min().y),
        },
        geo::Coordinate {
            x: a.max().x.max(b.max().x),
            y: a.max().y.max(b.max().y),
        },
    )
}
