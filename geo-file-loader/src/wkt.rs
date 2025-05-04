use std::io;

use geozero::ToGeoFeatures;

pub struct WktSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for WktSource {
    fn from_bytes(bytes: bytes::Bytes) -> Self {
        WktSource { bytes }
    }

    fn load(self) -> Result<crate::Features, crate::Error> {
        let mut bytes_cursor = io::Cursor::new(&self.bytes);
        let features = geozero::wkt::WktReader(&mut bytes_cursor)
            .to_geo_features()?
            .collect();
        Ok(features)
    }
}
