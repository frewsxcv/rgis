use std::io;

use geozero::GeozeroDatasource;

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

    fn load(self) -> Result<geo_features::FeatureCollection<f64>, crate::Error> {
        let mut bytes_cursor = io::Cursor::new(&self.bytes);
        let mut wkt_reader = geozero::wkt::WktReader(&mut bytes_cursor);
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        wkt_reader.process(&mut geo_writer)?;
        let geometry = geo_writer.take_geometry().ok_or(crate::Error::NoGeometry)?;
        Ok(geo_features::FeatureCollection::<f64>::from_geometry(
            geometry,
        ))
    }
}
