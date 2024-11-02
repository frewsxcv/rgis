pub struct WktSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for WktSource {
    fn from_bytes(bytes: bytes::Bytes) -> Self {
        WktSource { bytes }
    }

    fn load(self) -> Result<crate::LoadedFile, crate::Error> {
        let wkt = wkt::deserialize_wkt::<T = f64>(&self.bytes)?;
        Ok(crate::LoadedFile::Wkt(wkt))
        /*
        let mut bytes_cursor = io::Cursor::new(&self.bytes);
        let mut wkt_reader = geozero::wkt::WktReader(&mut bytes_cursor);
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
        wkt_reader.process(&mut geo_writer)?;
        let geometry = geometry_wrap::<f64, geo_projected::Unprojected>(
            geo_writer.take_geometry().ok_or(crate::Error::NoGeometry)?,
        );
        Ok(crate::LoadedFile::Geometry(geometry))
        */
    }
}
