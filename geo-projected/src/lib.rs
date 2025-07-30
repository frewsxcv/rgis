use geo::MapCoords;
use num_t::Num as TypedNum;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Projected;

#[derive(Debug)]
pub struct Unprojected;

pub type UnprojectedScalar = TypedNum<f64, Unprojected>;
pub type ProjectedScalar = TypedNum<f64, Projected>;

pub type ProjectedCoord<T = f64> = geo::Coord<TypedNum<T, Projected>>;
pub type UnprojectedCoord<T = f64> = geo::Coord<TypedNum<T, Unprojected>>;

pub trait WrapTo<Scalar: geo::CoordNum> {
    type Output<To: Debug>;
    fn wrap<To: Debug>(self) -> Self::Output<To>;
}

impl<Scalar: geo::CoordNum> WrapTo<Scalar> for geo::Coord<Scalar> {
    type Output<To: Debug> = geo::Coord<TypedNum<Scalar, To>>;
    #[inline]
    fn wrap<To: Debug>(self) -> Self::Output<To> {
        geo::Coord {
            x: TypedNum::<Scalar, To>::new(self.x),
            y: TypedNum::<Scalar, To>::new(self.y),
        }
    }
}

impl<Scalar: geo::CoordNum> WrapTo<Scalar> for geo::Rect<Scalar> {
    type Output<To: Debug> = geo::Rect<TypedNum<Scalar, To>>;
    #[inline]
    fn wrap<To: Debug>(self) -> Self::Output<To> {
        geo::Rect::new(self.min().wrap(), self.max().wrap())
    }
}

impl<Scalar: geo::CoordNum> WrapTo<Scalar> for geo::Geometry<Scalar> {
    type Output<To: Debug> = geo::Geometry<TypedNum<Scalar, To>>;
    #[inline]
    fn wrap<To: Debug>(self) -> Self::Output<To> {
        self.map_coords(move |coord| coord.wrap())
    }
}

impl<Scalar: geo::CoordNum> WrapTo<Scalar> for geo_features::Feature<Scalar> {
    type Output<To: Debug> = geo_features::Feature<TypedNum<Scalar, To>>;
    #[inline]
    fn wrap<To: Debug>(self) -> Self::Output<To> {
        geo_features::Feature {
            id: self.id,
            properties: self.properties,
            bounding_rect: self.bounding_rect.map(|rect| rect.wrap()),
            geometry: self.geometry.map(|geometry| geometry.wrap()),
        }
    }
}

impl<Scalar: geo::CoordNum> WrapTo<Scalar> for geo_features::FeatureCollection<Scalar> {
    type Output<To: Debug> = geo_features::FeatureCollection<TypedNum<Scalar, To>>;
    #[inline]
    fn wrap<To: Debug>(self) -> Self::Output<To> {
        geo_features::FeatureCollection {
            features: self
                .features
                .into_iter()
                .map(|feature| feature.wrap())
                .collect(),
            bounding_rect: self.bounding_rect.map(|rect| rect.wrap()),
        }
    }
}

pub trait CastTo<Scalar: geo::CoordNum, From: Debug> {
    type Output<To: Debug>;
    fn cast<To: Debug>(self) -> Self::Output<To>;
}

impl<Scalar: geo::CoordNum, From: Debug> CastTo<Scalar, From>
    for geo::Coord<TypedNum<Scalar, From>>
{
    type Output<To: Debug> = geo::Coord<TypedNum<Scalar, To>>;
    #[inline]
    fn cast<To: Debug>(self) -> Self::Output<To> {
        geo::Coord {
            x: TypedNum::<Scalar, To>::new(self.x.0),
            y: TypedNum::<Scalar, To>::new(self.y.0),
        }
    }
}

impl<Scalar: geo::CoordNum, From: Debug> CastTo<Scalar, From>
    for geo::Rect<TypedNum<Scalar, From>>
{
    type Output<To: Debug> = geo::Rect<TypedNum<Scalar, To>>;
    #[inline]
    fn cast<To: Debug>(self) -> Self::Output<To> {
        geo::Rect::new(self.min().cast(), self.max().cast())
    }
}

impl<Scalar: geo::CoordNum, From: Debug> CastTo<Scalar, From>
    for geo::Geometry<TypedNum<Scalar, From>>
{
    type Output<To: Debug> = geo::Geometry<TypedNum<Scalar, To>>;
    #[inline]
    fn cast<To: Debug>(self) -> Self::Output<To> {
        self.map_coords(move |coord| coord.cast())
    }
}

impl<Scalar: geo::CoordNum, From: Debug> CastTo<Scalar, From>
    for geo_features::Feature<TypedNum<Scalar, From>>
{
    type Output<To: Debug> = geo_features::Feature<TypedNum<Scalar, To>>;
    #[inline]
    fn cast<To: Debug>(self) -> Self::Output<To> {
        geo_features::Feature {
            id: self.id,
            properties: self.properties,
            bounding_rect: self.bounding_rect.map(|rect| rect.cast()),
            geometry: self.geometry.map(|geometry| geometry.cast()),
        }
    }
}

impl<Scalar: geo::CoordNum, From: Debug> CastTo<Scalar, From>
    for geo_features::FeatureCollection<TypedNum<Scalar, From>>
{
    type Output<To: Debug> = geo_features::FeatureCollection<TypedNum<Scalar, To>>;
    #[inline]
    fn cast<To: Debug>(self) -> Self::Output<To> {
        geo_features::FeatureCollection {
            features: self
                .features
                .into_iter()
                .map(|feature| feature.cast())
                .collect(),
            bounding_rect: self.bounding_rect.map(|rect| rect.cast()),
        }
    }
}
