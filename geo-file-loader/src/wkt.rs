use std::io;

use geozero::GeozeroDatasource;

pub struct WktSource {
    pub bytes: Vec<u8>,
}

impl crate::FileLoader for WktSource {
    type Error = geozero::error::GeozeroError;

    const FILE_TYPE_NAME: &'static str = "WKT";

    fn from_bytes(bytes: Vec<u8>) -> Self {
        WktSource { bytes }
    }

    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error> {
        let mut bytes_cursor = io::Cursor::new(&self.bytes);
        let mut wkt_reader = geozero::wkt::WktReader(&mut bytes_cursor);
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        wkt_reader.process(&mut geo_writer)?;
        let geometry = geo_writer.take_geometry().unwrap();
        Ok(geo_features::FeatureCollection::from_geometry(geometry).unwrap())
    }
}
