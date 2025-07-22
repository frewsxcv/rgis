use std::io;

use geozero::ToGeoFeatures;

pub struct WktSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for WktSource {
    type Output = crate::Features;

    fn from_bytes(bytes: bytes::Bytes) -> Self {
        WktSource { bytes }
    }

    fn load(self) -> Result<crate::Features, crate::Error> {
        let mut bytes_cursor = io::Cursor::new(&self.bytes);
        let features = geozero::wkt::WktReader(&mut bytes_cursor)
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
