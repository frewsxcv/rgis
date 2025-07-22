use geozero::ToGeoFeatures;
use std::io;

pub struct GeoJsonSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for GeoJsonSource {
    type Output = crate::Features;

    fn from_bytes(bytes: bytes::Bytes) -> Self {
        GeoJsonSource { bytes }
    }

    fn load(self) -> Result<crate::Features, crate::Error> {
        let bytes_cursor = io::Cursor::new(&self.bytes);
        let mut geojson_reader = geozero::geojson::GeoJsonReader(bytes_cursor);
        let features = geojson_reader
            .to_geo_features()?
            .map(|f| {
                let (geometry, properties) = f.into_inner();
                crate::Feature {
                    geometry,
                    properties,
                }
            })
            .collect();
        Ok(features)
    }
}
