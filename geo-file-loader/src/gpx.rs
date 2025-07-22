use geozero::ToGeoFeatures;
use std::io;

// TOOD: create generic file loader for geozero

pub struct GpxSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for GpxSource {
    type Output = crate::Features;

    fn from_bytes(bytes: bytes::Bytes) -> Self {
        GpxSource { bytes }
    }

    fn load(self) -> Result<crate::Features, crate::Error> {
        let bytes_cursor = io::Cursor::new(&self.bytes);
        let features = geozero::gpx::GpxReader(bytes_cursor)
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
