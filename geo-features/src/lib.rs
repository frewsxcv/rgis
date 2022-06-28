use geo::{BoundingRect, Contains};
use std::{collections, fmt};

#[derive(Clone, Debug)]
pub struct Feature {
    pub geometry: geo::Geometry,
    pub properties: collections::HashMap<String, Value>,
    pub bounding_rect: geo::Rect,
}

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl Feature {
    pub fn from_geometry(
        geometry: geo::Geometry,
        properties: collections::HashMap<String, Value>,
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

impl Contains<geo::Coordinate> for Feature {
    fn contains(&self, coord: &geo::Coordinate) -> bool {
        self.bounding_rect.contains(coord) && self.geometry.contains(coord)
    }
}

#[derive(Clone, Debug)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
    pub bounding_rect: geo::Rect,
}

#[derive(Debug)]
pub struct BoundingRectError;

impl fmt::Display for BoundingRectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not generate bounding rect")
    }
}

impl std::error::Error for BoundingRectError {}

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

    pub fn to_geometry_collection(&self) -> geo::GeometryCollection {
        geo::GeometryCollection(
            self.features
                .iter()
                .map(|f| f.geometry.clone())
                .collect::<Vec<_>>(),
        )
    }

    pub fn bounding_rect(&self) -> Result<geo::Rect, BoundingRectError> {
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

fn bounding_rect_from_features(features: &[Feature]) -> geo::Rect {
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
