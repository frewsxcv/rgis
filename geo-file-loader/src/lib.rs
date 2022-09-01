#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

pub mod geojson;
pub mod wkt;

pub trait FileLoader {
    type Error;

    const FILE_TYPE_NAME: &'static str;

    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error>;
}
