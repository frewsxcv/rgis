use std::io;

use geozero::ToGeoFeatures;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Geozero(#[from] geozero::error::GeozeroError),
}

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
