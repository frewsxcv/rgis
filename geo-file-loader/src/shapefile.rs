use crate::convert_geometry_to_unprojected;
use std::io;

pub struct ShapefileSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for ShapefileSource {
    fn from_bytes(bytes: bytes::Bytes) -> Self {
        ShapefileSource { bytes }
    }

    fn load(
        self,
    ) -> Result<geo_features::FeatureCollection<geo_projected::UnprojectedScalar>, crate::Error>
    {
        let mut bytes_cursor = io::Cursor::new(&self.bytes);
        let shapefile_reader = geozero::shp::ShpReader::new(&mut bytes_cursor)?;
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        for _ in shapefile_reader.iter_geometries(&mut geo_writer) {}
        let geometry = geo_writer.take_geometry().ok_or(crate::Error::NoGeometry)?;
        let geometry = convert_geometry_to_unprojected(geometry);
        Ok(geo_features::FeatureCollection::from_geometry(geometry))
    }
}
