#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use geo::{BoundingRect, Contains, ConvexHull};
use std::{collections, fmt};

#[derive(Clone, Debug)]
pub struct Feature {
    pub geometry: Option<geo::Geometry>,
    pub properties: Properties,
    pub bounding_rect: Option<geo::Rect>,
}

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub type Properties = collections::HashMap<String, Value>;

impl Feature {
    pub fn from_geometry(
        geometry: Option<geo::Geometry>,
        properties: Properties,
    ) -> Result<Self, BoundingRectError> {
        let bounding_rect = geometry
            .as_ref()
            .and_then(|geometry| geometry.bounding_rect());

        Ok(Feature {
            geometry,
            properties,
            bounding_rect,
        })
    }

    pub fn recalculate_bounding_rect(&mut self) -> Result<(), BoundingRectError> {
        self.bounding_rect = self
            .geometry
            .as_ref()
            .and_then(|geometry| geometry.bounding_rect());
        Ok(())
    }
}

impl Contains<geo::Coordinate> for Feature {
    fn contains(&self, coord: &geo::Coordinate) -> bool {
        self.bounding_rect
            .as_ref()
            .map(|bounding_rect| bounding_rect.contains(coord))
            .unwrap_or(false)
            && self
                .geometry
                .as_ref()
                .map(|geometry| geometry.contains(coord))
                .unwrap_or(false)
    }
}

#[derive(Clone, Debug)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
    pub bounding_rect: Option<geo::Rect>,
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
    pub fn from_geometry(geometry: geo::Geometry) -> Result<Self, BoundingRectError> {
        let feature = Feature::from_geometry(Some(geometry), Default::default())?;
        Ok(Self::from_feature(feature))
    }

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
                .filter_map(|f| f.geometry.clone())
                .collect::<Vec<_>>(),
        )
    }

    pub fn bounding_rect(&self) -> Result<geo::Rect, BoundingRectError> {
        rect_merge_many(
            self.features
                .iter()
                .filter_map(|feature| feature.geometry.as_ref())
                .filter_map(|geometry| geometry.bounding_rect()),
        )
    }

    pub fn convex_hull(&self) -> geo::Polygon {
        self.to_geometry_collection().convex_hull()
        // let mut hulls = vec![];
        // for feature in &self.features {
        //     hulls.push(feature.geometry.as_ref().unwrap().convex_hull());
        // }
        // geo::MultiPolygon::new(hulls)
    }

    pub fn recalculate_bounding_rect(&mut self) -> Result<(), BoundingRectError> {
        self.bounding_rect = bounding_rect_from_features(&self.features);
        Ok(())
    }
}

fn bounding_rect_from_features(features: &[Feature]) -> Option<geo::Rect> {
    features
        .iter()
        .map(|feature| feature.bounding_rect)
        .fold(None, option_rect_merge)
}

// TODO: this assumes the iterator has one item. is that okay?
fn rect_merge_many<T: geo::CoordFloat>(
    mut iter: impl Iterator<Item = geo::Rect<T>>,
) -> Result<geo::Rect<T>, BoundingRectError> {
    let first = iter.next().ok_or(BoundingRectError)?;
    Ok(iter.fold(first, rect_merge))
}

fn option_rect_merge<T: geo::CoordFloat>(
    a: Option<geo::Rect<T>>,
    b: Option<geo::Rect<T>>,
) -> Option<geo::Rect<T>> {
    match (a, b) {
        (Some(a), Some(b)) => Some(rect_merge(a, b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
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
