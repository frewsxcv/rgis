use crate::convert_geometry_to_unprojected;
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

    fn load(
        self,
    ) -> Result<geo_features::FeatureCollection<geo_projected::UnprojectedScalar>, crate::Error>
    {
        let mut bytes_cursor = io::Cursor::new(&self.bytes);
        let mut wkt_reader = geozero::wkt::WktReader(&mut bytes_cursor);
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        wkt_reader.process(&mut geo_writer)?;
        let geometry = convert_geometry_to_unprojected(
            geo_writer.take_geometry().ok_or(crate::Error::NoGeometry)?,
        );
        Ok(geo_features::FeatureCollection::from_geometry(geometry))
    }
}
