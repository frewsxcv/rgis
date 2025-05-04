#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

mod geojson;
mod gpx;
mod shapefile;
mod wkt;

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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Geozero(#[from] geozero::error::GeozeroError),
    #[error("{0}")]
    Shapefile(#[from] geozero::shp::Error),
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

#[derive(Clone, Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub type Feature = geozero::geo_types::GeoFeature;

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
