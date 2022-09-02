#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

mod geojson;
mod wkt;

pub use crate::geojson::GeoJsonSource;
pub use crate::wkt::WktSource;

pub trait FileLoader {
    type Error;

    const FILE_TYPE_NAME: &'static str;

    fn from_bytes(bytes: Vec<u8>) -> Self;
    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error>;
}
