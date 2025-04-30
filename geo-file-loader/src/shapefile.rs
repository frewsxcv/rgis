use std::io;

pub struct ShapefileSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for ShapefileSource {
    fn from_bytes(bytes: bytes::Bytes) -> Self {
        ShapefileSource { bytes }
    }

    fn load(self) -> Result<geo_features::FeatureCollection<f64>, crate::Error> {
        let mut bytes_cursor = io::Cursor::new(&self.bytes);
        let shapefile_reader = geozero::shp::ShpReader::new(&mut bytes_cursor)?;
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        for _ in shapefile_reader.iter_geometries(&mut geo_writer) {}
        let geometry = geo_writer.take_geometry().ok_or(crate::Error::NoGeometry)?;
        Ok(geo_features::FeatureCollection::from_geometry(geometry))
    }
}
