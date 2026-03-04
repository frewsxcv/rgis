mod geotiff;

pub use geotiff::GeoTiffSource;

#[derive(Clone, Debug)]
pub struct Raster {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub format: RasterFormat,
    pub extent: geo_types::Rect<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RasterFormat {
    R8,
    Rgba8,
}

#[derive(Debug)]
pub enum Error {
    Tiff(tiff::TiffError),
    UnsupportedColorType,
    MissingGeoInfo,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Tiff(err) => write!(f, "TIFF error: {err}"),
            Error::UnsupportedColorType => write!(f, "Unsupported color type"),
            Error::MissingGeoInfo => write!(f, "Missing geo-referencing information"),
        }
    }
}

impl From<tiff::TiffError> for Error {
    fn from(err: tiff::TiffError) -> Self {
        Error::Tiff(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_uint8_rgb_deflate() {
        let bytes = bytes::Bytes::from(
            std::fs::read(
                "../geotiff-test-data/rasterio_generated/fixtures/uint8_rgb_deflate_block64_cog.tif",
            )
            .expect("read test file"),
        );
        let raster = GeoTiffSource::from_bytes(bytes).load().expect("load GeoTIFF");
        assert!(raster.width > 0);
        assert!(raster.height > 0);
        assert_eq!(raster.format, RasterFormat::Rgba8);
    }
}
