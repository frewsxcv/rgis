use geozero::GeozeroDatasource;
use std::io;

pub struct GeoJsonSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for GeoJsonSource {
    fn from_bytes(bytes: bytes::Bytes) -> Self {
        GeoJsonSource { bytes }
    }

    fn load(self) -> Result<geo_features::FeatureCollection<f64>, crate::Error> {
        let bytes_cursor = io::Cursor::new(&self.bytes);
        let mut geojson_reader = geozero::geojson::GeoJsonReader(bytes_cursor);
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        geojson_reader.process(&mut geo_writer)?;
        let geometry = geo_writer.take_geometry().ok_or(crate::Error::NoGeometry)?;
        Ok(geo_features::FeatureCollection::from_geometry(geometry))
    }
}
