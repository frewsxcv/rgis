/*
struct WktSource;

impl crate::FileLoader for WktSource {
    type Error = LoadGeoJsonError;

    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error> {
        Ok(match self {
            #[cfg(not(target_arch = "wasm32"))]
            GeoJsonSource::Path(path) => load_from_path(&path)?,
            GeoJsonSource::Bytes(bytes) => load_from_reader(io::Cursor::new(bytes))?,
        })
    }
}
*/
