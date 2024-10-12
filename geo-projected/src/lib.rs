#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use geo::{CoordNum, MapCoords};
use std::fmt::Debug;
use typed_num::TypedNum;

#[derive(Debug)]
pub struct Projected;

#[derive(Debug)]
pub struct Unprojected;

pub type UnprojectedScalar = TypedNum<f64, Unprojected>;
pub type ProjectedScalar = TypedNum<f64, Projected>;

pub type ProjectedCoord<T = f64> = geo::Coord<TypedNum<T, Projected>>;
pub type UnprojectedCoord<T = f64> = geo::Coord<TypedNum<T, Unprojected>>;

#[inline]
fn coord_cast<Scalar: geo::CoordNum, From: Debug, To: Debug>(
    coord: geo::Coord<TypedNum<Scalar, From>>,
) -> geo::Coord<TypedNum<Scalar, To>> {
    geo::Coord {
        x: TypedNum::<Scalar, To>::new(coord.x.0),
        y: TypedNum::<Scalar, To>::new(coord.y.0),
    }
}

#[inline]
fn rect_cast<Scalar: geo::CoordNum, From: Debug, To: Debug>(
    rect: geo::Rect<TypedNum<Scalar, From>>,
) -> geo::Rect<TypedNum<Scalar, To>> {
    geo::Rect::new(coord_cast(rect.min()), coord_cast(rect.max()))
}

#[inline]
pub fn geometry_cast<Scalar: geo::CoordNum, From: Debug, To: Debug>(
    geometry: geo::Geometry<TypedNum<Scalar, From>>,
) -> geo::Geometry<TypedNum<Scalar, To>> {
    geometry.map_coords(move |coord| coord_cast(coord))
}

#[inline]
pub fn feature_cast<Scalar: geo::CoordNum, From: Debug, To: Debug>(
    feature: geo_features::Feature<TypedNum<Scalar, From>>,
) -> geo_features::Feature<TypedNum<Scalar, To>> {
    geo_features::Feature {
        id: feature.id,
        properties: feature.properties,
        bounding_rect: feature.bounding_rect.map(rect_cast),
        geometry: feature.geometry.map(geometry_cast),
    }
}

#[inline]
pub fn feature_collection_cast<Scalar: geo::CoordNum, From: Debug, To: Debug>(
    feature_collection: geo_features::FeatureCollection<TypedNum<Scalar, From>>,
) -> geo_features::FeatureCollection<TypedNum<Scalar, To>> {
    geo_features::FeatureCollection {
        features: feature_collection
            .features
            .into_iter()
            .map(feature_cast)
            .collect(),
        bounding_rect: feature_collection.bounding_rect.map(rect_cast),
    }
}
