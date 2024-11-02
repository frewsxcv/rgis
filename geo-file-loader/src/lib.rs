#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

mod geojson_loader;
mod gpx_loader;
mod shapefile_loader;
mod wkt_loader;

pub use crate::geojson_loader::GeoJsonSource;
pub use crate::gpx_loader::GpxSource;
pub use crate::shapefile_loader::ShapefileSource;
pub use crate::wkt_loader::WktSource;
use enum_delegate::delegate;
use geo_traits::GeometryTrait;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileFormat {
    GeoJson,
    Shapefile,
    Wkt,
    Gpx,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    GeoJson(#[from] geojson::Error),
    #[error("No geometry found in file")]
    NoGeometry,
}

impl FileFormat {
    pub const fn is_plaintext(self) -> bool {
        match self {
            Self::GeoJson => true,
            Self::Gpx => true,
            Self::Shapefile => false,
            Self::Wkt => true,
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::GeoJson => "GeoJSON",
            Self::Gpx => "GPX",
            Self::Shapefile => "Shapefile",
            Self::Wkt => "WKT",
        }
    }
}

pub fn load_file(file_format: FileFormat, bytes: bytes::Bytes) -> Result<LoadedFile, Error> {
    match file_format {
        FileFormat::GeoJson => Ok(GeoJsonSource::from_bytes(bytes).load()?),
        FileFormat::Gpx => Ok(GpxSource::from_bytes(bytes).load()?),
        FileFormat::Shapefile => Ok(ShapefileSource::from_bytes(bytes).load()?),
        FileFormat::Wkt => Ok(WktSource::from_bytes(bytes).load()?),
    }
}

// Define "either" type that can be either a GeoJson or Wkt associated types
#[derive(Debug, PartialEq, Copy, Clone, geo_traits::PointTrait)]
pub enum Either<X, Y> {
    GeoJson(X),
    Wkt(Y),
}

impl<X, Y> PartialOrd for Either<X, Y>
where
    X: PartialOrd,
    Y: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl<X, Y> num_traits::ToPrimitive for Either<X, Y>
where
    X: num_traits::ToPrimitive,
    Y: num_traits::ToPrimitive,
{
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
}

impl<X, Y> num_traits::NumCast for Either<X, Y>
where
    X: num_traits::NumCast,
    Y: num_traits::NumCast,
{
    fn from(n: Self) -> Option<f64> {
        todo!()
    }
}

impl<X, Y> num_traits::Num for Either<X, Y>
where
    X: num_traits::Num,
    Y: num_traits::Num,
{
    type FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl<X, Y> geo_traits::CoordTrait for Either<X, Y>
where
    X: geo_traits::CoordTrait,
    Y: geo_traits::CoordTrait,
{
    type T = Either<X::T, Y::T>;

    fn dim(&self) -> geo_traits::Dimensions {
        match self {
            Self::GeoJson(geojson) => geojson.dim(),
            Self::Wkt(wkt) => wkt.dim(),
        }
    }

    fn x(&self) -> Self::T {
        match self {
            Self::GeoJson(geojson) => Either::GeoJson(geojson.x()),
            Self::Wkt(wkt) => Either::Wkt(wkt.x()),
        }
    }

    fn y(&self) -> Self::T {
        match self {
            Self::GeoJson(geojson) => Either::GeoJson(geojson.y()),
            Self::Wkt(wkt) => Either::Wkt(wkt.y()),
        }
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        todo!()
    }
}

impl<X, Y> geo_traits::PointTrait for Either<X, Y>
where
    X: geo_traits::PointTrait,
    Y: geo_traits::PointTrait,
{
    type T = X::T;
    type CoordType<'a>
        = Either<X::CoordType<'a>, Y::CoordType<'a>>
    where
        X: 'a,
        Y: 'a;

    fn dim(&self) -> geo_traits::Dimensions {
        match self {
            Self::GeoJson(geojson) => geojson.dim(),
            Self::Wkt(wkt) => wkt.dim(),
        }
    }
    fn coord(&self) -> Option<Self::CoordType<'_>> {
        match self {
            Self::GeoJson(geojson) => geojson.coord().map(Either::GeoJson),
            Self::Wkt(wkt) => wkt.coord().map(Either::Wkt),
        }
    }
}

#[delegate(for(LastName))]
pub enum LoadedFile {
    GeoJson(geojson::GeoJson),
    Wkt(wkt::Wkt<f64>),
}

impl geo_traits::GeometryTrait for LoadedFile {
    type T = f64;
    type PointType<'a> = Either<
        <geojson::GeoJson as geo_traits::GeometryTrait>::PointType<'a>,
        <wkt::Wkt<f64> as geo_traits::GeometryTrait>::PointType<'a>,
    >;
    type LineStringType<'a> = <geojson::GeoJson as geo_traits::GeometryTrait>::LineStringType<'a>;
    type PolygonType<'a> = <geojson::GeoJson as geo_traits::GeometryTrait>::PolygonType<'a>;
    type MultiPointType<'a> = <geojson::GeoJson as geo_traits::GeometryTrait>::MultiPointType<'a>;
    type MultiLineStringType<'a> =
        <geojson::GeoJson as geo_traits::GeometryTrait>::MultiLineStringType<'a>;
    type MultiPolygonType<'a> =
        <geojson::GeoJson as geo_traits::GeometryTrait>::MultiPolygonType<'a>;
    type GeometryCollectionType<'a> =
        <geojson::GeoJson as geo_traits::GeometryTrait>::GeometryCollectionType<'a>;
    type RectType<'a> = geo_traits::UnimplementedRect<Self::T>;
    type TriangleType<'a> = geo_traits::UnimplementedTriangle<Self::T>;
    type LineType<'a> = geo_traits::UnimplementedLine<Self::T>;

    fn dim(&self) -> geo_traits::Dimensions {
        // FIXME
        geo_traits::Dimensions::Xy
    }
    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Self::PointType<'_>,
        Self::LineStringType<'_>,
        Self::PolygonType<'_>,
        Self::MultiPointType<'_>,
        Self::MultiLineStringType<'_>,
        Self::MultiPolygonType<'_>,
        Self::GeometryCollectionType<'_>,
        Self::RectType<'_>,
        Self::TriangleType<'_>,
        Self::LineType<'_>,
    > {
        match self {
            Self::GeoJson(geojson) => geojson.as_type(),
            Self::Wkt(wkt) => wkt.as_type(),
        }
    }
}

trait FileLoader {
    fn from_bytes(bytes: bytes::Bytes) -> Self;
    fn load(self) -> Result<LoadedFile, Error>;
}
