#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

#[derive(Clone, Debug)]
pub enum RasterFormat {
    R8,
    Rgba8,
}

#[derive(Clone, Debug)]
pub struct Raster {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub format: RasterFormat,
}
