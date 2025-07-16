use geozero::ToGeoFeatures;
use std::io;

// TOOD: create generic file loader for geozero

pub struct GpxSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for GpxSource {
    fn from_bytes(bytes: bytes::Bytes) -> Self {
        GpxSource { bytes }
    }

    fn load(self) -> Result<crate::Features, crate::Error> {
        let bytes_cursor = io::Cursor::new(&self.bytes);

        let reader = geozero::gpx::GpxReader(bytes_cursor);

        use geoarrow::io::geozero::ToGeometry;

        let meow = reader.to_geometry()?;

        // let features = geozero::gpx::GpxReader(bytes_cursor)
        //     .to_geo_features()?
        //     .collect();

        // Ok(features)

        todo!()
    }
}
