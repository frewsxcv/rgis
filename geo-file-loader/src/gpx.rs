use std::io;

use geo::MapCoords;
use geozero::GeozeroDatasource;

// TOOD: create generic file loader for geozero

pub struct GpxSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for GpxSource {
    fn from_bytes(bytes: bytes::Bytes) -> Self {
        GpxSource { bytes }
    }

    fn load(
        self,
    ) -> Result<geo_features::FeatureCollection<geo_projected::UnprojectedScalar>, crate::Error>
    {
        let bytes_cursor = io::Cursor::new(&self.bytes);
        let mut gpx_reader = geozero::gpx::GpxReader(bytes_cursor);
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        gpx_reader.process(&mut geo_writer)?;
        let geometry = geo_writer.take_geometry().ok_or(crate::Error::NoGeometry)?;
        let geometry = geometry.map_coords(|coord| geo_projected::UnprojectedCoord {
            x: geo_projected::UnprojectedScalar::new(coord.x),
            y: geo_projected::UnprojectedScalar::new(coord.y),
        });
        Ok(geo_features::FeatureCollection::from_geometry(geometry))
    }
}
