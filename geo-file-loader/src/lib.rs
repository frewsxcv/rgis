mod geojson;
mod gpx;
mod shapefile;
mod wkt;

use geozero::geo_types::GeoProperties;

pub use crate::geojson::GeoJsonSource;
pub use crate::gpx::GpxSource;
pub use crate::shapefile::ShapefileSource;
pub use crate::wkt::WktSource;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileFormat {
    GeoJson,
    Shapefile,
    Wkt,
    Gpx,
}

#[derive(Debug)]
pub enum Error {
    Geozero(geozero::error::GeozeroError),
    Shapefile(geozero::shp::Error),
    NoGeometry,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Geozero(err) => write!(f, "{}", err),
            Error::Shapefile(err) => write!(f, "{}", err),
            Error::NoGeometry => write!(f, "No geometry found in file"),
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

pub fn load_file(file_format: FileFormat, bytes: bytes::Bytes) -> Result<Features, Error> {
    match file_format {
        FileFormat::GeoJson => Ok(GeoJsonSource::from_bytes(bytes).load()?),
        FileFormat::Gpx => Ok(GpxSource::from_bytes(bytes).load()?),
        FileFormat::Shapefile => Ok(ShapefileSource::from_bytes(bytes).load()?),
        FileFormat::Wkt => Ok(WktSource::from_bytes(bytes).load()?),
    }
}

trait FileLoader {
    fn from_bytes(bytes: bytes::Bytes) -> Self;
    fn load(self) -> Result<Features, Error>;
}
