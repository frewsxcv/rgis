use crate::LoadedFile;
use geojson::GeoJson;
use std::io;

pub struct GeoJsonSource {
    pub bytes: bytes::Bytes,
}

impl crate::FileLoader for GeoJsonSource {
    fn from_bytes(bytes: bytes::Bytes) -> Self {
        GeoJsonSource { bytes }
    }

    fn load(self) -> Result<LoadedFile, crate::Error> {
        let bytes_cursor = io::Cursor::new(&self.bytes);
        let geojson =
            GeoJson::from_reader(bytes_cursor).map_err(|e| geojson::Error::MalformedJson(e))?;
        Ok(LoadedFile::GeoJson(geojson))
    }
}
