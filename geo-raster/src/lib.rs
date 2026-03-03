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
    fn test_load_sample_geotiff() {
        let bytes =
            bytes::Bytes::from(std::fs::read("../test-data/sample.tif").expect("read test file"));
        let raster = GeoTiffSource::from_bytes(bytes).load().expect("load GeoTIFF");
        assert_eq!(raster.width, 32);
        assert_eq!(raster.height, 32);
        assert_eq!(raster.format, RasterFormat::Rgba8);
        assert_eq!(raster.data.len(), 32 * 32 * 4);
        // Check extent: origin (-10, 10), scale 0.625, size 32x32
        // min_x = -10, max_x = -10 + 0.625*32 = 10
        // max_y = 10, min_y = 10 - 0.625*32 = -10
        let ext = &raster.extent;
        assert!((ext.min().x - (-10.0)).abs() < 1e-6);
        assert!((ext.min().y - (-10.0)).abs() < 1e-6);
        assert!((ext.max().x - 10.0).abs() < 1e-6);
        assert!((ext.max().y - 10.0).abs() < 1e-6);
    }
}
