#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use std::fmt;

mod geojson;
mod gpx;
mod shapefile;
mod wkt;

pub use crate::geojson::GeoJsonSource;
pub use crate::shapefile::ShapefileSource;
pub use crate::wkt::WktSource;

pub trait FileLoader {
    type Error: fmt::Debug;

    const FILE_TYPE_NAME: &'static str;

    fn from_bytes(bytes: bytes::Bytes) -> Self;
    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error>;
}
