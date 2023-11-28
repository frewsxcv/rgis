use std::io;

use geozero::GeozeroDatasource;

pub struct GeoJsonSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for GeoJsonSource {
    type Error = Error;

    const FILE_TYPE_NAME: &'static str = "GeoJSON";

    fn from_bytes(bytes: bytes::Bytes) -> Self {
        GeoJsonSource { bytes }
    }

    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error> {
        let bytes_cursor = io::Cursor::new(&self.bytes);
        let mut geojson_reader = geozero::geojson::GeoJsonReader(bytes_cursor);
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        geojson_reader.process(&mut geo_writer)?;
        let geometry = geo_writer.take_geometry().ok_or(Error::NoGeometry)?;
        Ok(geo_features::FeatureCollection::from_geometry(geometry))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Geozero(#[from] geozero::error::GeozeroError),
    #[error("No geometry found in GeoJSON file")]
    NoGeometry,
}
