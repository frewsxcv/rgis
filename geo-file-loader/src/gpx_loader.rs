pub struct GpxSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for GpxSource {
    fn from_bytes(bytes: bytes::Bytes) -> Self {
        GpxSource { bytes }
    }

    fn load(self) -> Result<crate::LoadedFile, crate::Error> {
        unimplemented!()
        /*
        let bytes_cursor = io::Cursor::new(&self.bytes);
        let mut gpx_reader = geozero::gpx::GpxReader(bytes_cursor);
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        gpx_reader.process(&mut geo_writer)?;
        let geometry = geo_writer.take_geometry().ok_or(crate::Error::NoGeometry)?;
        let geometry = geometry_wrap::<f64, geo_projected::Unprojected>(geometry);
        Ok(crate::LoadedFile::Geometry(geometry))
        */
    }
}
