use std::io;

use geozero::GeozeroDatasource;

struct WktSource {
    bytes: Vec<u8>,
}

impl crate::FileLoader for WktSource {
    type Error = ();

    const FILE_TYPE_NAME: &'static str = "WKT";

    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error> {
        let mut bytes_cursor = io::Cursor::new(&self.bytes);
        let mut wkt_reader = geozero::wkt::WktReader(&mut bytes_cursor);
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        wkt_reader.process(&mut geo_writer).unwrap();
        let geometry = geo_writer.take_geometry().unwrap();
        Ok(geo_features::FeatureCollection::from_geometry(geometry).unwrap())
    }
}
