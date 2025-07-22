#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

mod geojson;
mod geotiff;
mod gpx;
mod shapefile;
mod wkt;

use geo_raster::Raster;
use geozero::geo_types::GeoProperties;

pub use crate::geojson::GeoJsonSource;
pub use crate::geotiff::GeoTiffSource;
pub use crate::gpx::GpxSource;
pub use crate::shapefile::ShapefileSource;
pub use crate::wkt::WktSource;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileFormat {
    GeoJson,
    Shapefile,
    Wkt,
    Gpx,
    GeoTiff,
}

#[derive(Debug)]
pub enum Error {
    Geozero(geozero::error::GeozeroError),
    Shapefile(geozero::shp::Error),
    NoGeometry,
    GeoTiff(tiff::TiffError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Geozero(err) => write!(f, "{}", err),
            Error::Shapefile(err) => write!(f, "{}", err),
            Error::NoGeometry => write!(f, "No geometry found in file"),
            Error::GeoTiff(err) => write!(f, "{}", err),
        }
    }
}

impl From<geozero::error::GeozeroError> for Error {
    fn from(err: geozero::error::GeozeroError) -> Self {
        Error::Geozero(err)
    }
}

impl From<geozero::shp::Error> for Error {
    fn from(err: geozero::shp::Error) -> Self {
        Error::Shapefile(err)
    }
}

impl From<tiff::TiffError> for Error {
    fn from(err: tiff::TiffError) -> Self {
        Error::GeoTiff(err)
    }
}

impl FileFormat {
    pub const fn is_plaintext(self) -> bool {
        match self {
            Self::GeoJson => true,
            Self::Gpx => true,
            Self::Shapefile => false,
            Self::Wkt => true,
            Self::GeoTiff => false,
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::GeoJson => "GeoJSON",
            Self::Gpx => "GPX",
            Self::Shapefile => "Shapefile",
            Self::Wkt => "WKT",
            Self::GeoTiff => "GeoTIFF",
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

#[derive(Debug, PartialEq)]
pub struct Feature {
    pub geometry: geo::Geometry,
    pub properties: GeoProperties,
}

pub type Features = Vec<Feature>;

pub type OwnedColumnValue = geozero::geo_types::OwnedColumnValue;

pub fn load_vector_file(file_format: FileFormat, bytes: bytes::Bytes) -> Result<Features, Error> {
    match file_format {
        FileFormat::GeoJson => Ok(GeoJsonSource::from_bytes(bytes).load()?),
        FileFormat::Gpx => Ok(GpxSource::from_bytes(bytes).load()?),
        FileFormat::Shapefile => Ok(ShapefileSource::from_bytes(bytes).load()?),
        FileFormat::Wkt => Ok(WktSource::from_bytes(bytes).load()?),
        FileFormat::GeoTiff => panic!("GeoTIFF is not a vector file format"),
    }
}

pub fn load_raster_file(file_format: FileFormat, bytes: bytes::Bytes) -> Result<Raster, Error> {
    match file_format {
        FileFormat::GeoTiff => Ok(GeoTiffSource::from_bytes(bytes).load()?),
        _ => panic!("Unsupported raster file format"),
    }
}

trait FileLoader {
    type Output;
    fn from_bytes(bytes: bytes::Bytes) -> Self;
    fn load(self) -> Result<Self::Output, Error>;
}
