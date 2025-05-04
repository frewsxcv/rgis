use geozero::ToGeoFeatures;
use std::io;

pub struct GeoJsonSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for GeoJsonSource {
    fn from_bytes(bytes: bytes::Bytes) -> Self {
        GeoJsonSource { bytes }
    }

    fn load(self) -> Result<crate::Features, crate::Error> {
        let bytes_cursor = io::Cursor::new(&self.bytes);
        let mut geojson_reader = geozero::geojson::GeoJsonReader(bytes_cursor);
        let features = geojson_reader.to_geo_features()?.collect();
        Ok(features)
    }
}
