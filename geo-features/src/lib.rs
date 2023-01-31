#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use geo::{BoundingRect, Contains};
use std::{collections, fmt, iter, num, sync};

#[derive(Default)]
pub struct FeatureBuilder {
    geometry: Option<geo::Geometry>,
    properties: Properties,
}

impl FeatureBuilder {
    pub fn new() -> Self {
        FeatureBuilder {
            ..Default::default()
        }
    }

    pub fn with_geometry(self, geometry: geo::Geometry) -> Self {
        FeatureBuilder {
            geometry: Some(geometry),
            ..self
        }
    }

    pub fn with_properties(self, properties: Properties) -> Self {
        FeatureBuilder { properties, ..self }
    }

    pub fn build(self) -> Result<Feature, BoundingRectError> {
        let bounding_rect = self
            .geometry
            .as_ref()
            .and_then(|geometry| geometry.bounding_rect());
        Ok(Feature {
            id: FeatureId::new(),
            geometry: self.geometry,
            properties: self.properties,
            bounding_rect,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct Feature {
    pub id: FeatureId,
    pub geometry: Option<geo::Geometry>,
    pub properties: Properties,
    pub bounding_rect: Option<geo::Rect>,
}

impl<'a> geo::CoordsIter<'a> for Feature {
    type Scalar = f64;
    type Iter = Box<dyn Iterator<Item = geo::Coord<Self::Scalar>> + 'a>;
    type ExteriorIter = Box<dyn Iterator<Item = geo::Coord<Self::Scalar>> + 'a>;

    fn coords_count(&'a self) -> usize {
        self.geometry
            .as_ref()
            .map(|g| g.coords_count())
            .unwrap_or(0)
    }

    fn coords_iter(&'a self) -> Self::Iter {
        match self.geometry {
            Some(ref g) => Box::new(g.coords_iter()),
            None => Box::new(iter::empty()),
        }
    }

    fn exterior_coords_iter(&'a self) -> Self::ExteriorIter {
        match self.geometry {
            Some(ref g) => Box::new(g.exterior_coords_iter()),
            None => Box::new(iter::empty()),
        }
    }
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
    pub fn recalculate_bounding_rect(&mut self) -> Result<(), BoundingRectError> {
        self.bounding_rect = self
            .geometry
            .as_ref()
            .and_then(|geometry| geometry.bounding_rect());
        Ok(())
    }
}

impl<G> Contains<G> for Feature
where
    geo::Rect: Contains<G>,
    geo::Geometry: Contains<G>,
{
    fn contains(&self, coord: &G) -> bool {
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

#[derive(Default, Clone, Debug)]
pub struct FeatureCollection {
    pub features: Vec<Feature>,
    pub bounding_rect: Option<geo::Rect>,
}

impl FeatureCollection {
    pub fn new() -> Self {
        FeatureCollection::default()
    }
}

impl<'a> geo::CoordsIter<'a> for FeatureCollection {
    type Scalar = f64;
    type Iter = iter::Empty<geo::Coord<Self::Scalar>>;
    type ExteriorIter = iter::Empty<geo::Coord<Self::Scalar>>;

    fn coords_count(&'a self) -> usize {
        self.features.iter().map(|f| f.coords_count()).sum()
    }

    fn coords_iter(&'a self) -> Self::Iter {
        todo!()
    }

    fn exterior_coords_iter(&'a self) -> Self::ExteriorIter {
        todo!()
    }
}

impl<G> Contains<G> for FeatureCollection
where
    geo::Rect: Contains<G>,
    geo::Geometry: Contains<G>,
{
    fn contains(&self, coord: &G) -> bool {
        self.bounding_rect
            .as_ref()
            .map(|bounding_rect| bounding_rect.contains(coord))
            .unwrap_or(false)
            && self.features.iter().any(|feature| {
                feature
                    .geometry
                    .as_ref()
                    .map(|geometry| geometry.contains(coord))
                    .unwrap_or(false)
            })
    }
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
        let feature = FeatureBuilder::new().with_geometry(geometry).build()?;
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

    pub fn geometry_iter(&self) -> impl Iterator<Item = &geo::Geometry> {
        self.features.iter().filter_map(|f| f.geometry.as_ref())
    }

    pub fn to_geometry_collection(&self) -> geo::GeometryCollection {
        geo::GeometryCollection(self.geometry_iter().cloned().collect())
    }

    pub fn bounding_rect(&self) -> Result<geo::Rect, BoundingRectError> {
        rect_merge_many(
            self.geometry_iter()
                .filter_map(|geometry| geometry.bounding_rect()),
        )
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
        geo::Coord {
            x: a.min().x.min(b.min().x),
            y: a.min().y.min(b.min().y),
        },
        geo::Coord {
            x: a.max().x.max(b.max().x),
            y: a.max().y.max(b.max().y),
        },
    )
}

// The starting value is `1` so we can utilize `NonZeroU16`.
static NEXT_ID: sync::atomic::AtomicU16 = sync::atomic::AtomicU16::new(1);

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct FeatureId(num::NonZeroU16);

impl Default for FeatureId {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureId {
    pub fn new() -> Self {
        FeatureId(new_id())
    }
}

fn new_id() -> num::NonZeroU16 {
    // Unsafety: The starting ID is 1 and we always increment.
    unsafe { num::NonZeroU16::new_unchecked(NEXT_ID.fetch_add(1, sync::atomic::Ordering::SeqCst)) }
}
