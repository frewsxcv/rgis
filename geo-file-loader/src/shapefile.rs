use std::io;

use geozero::GeozeroDatasource;

pub struct ShapefileSource {
    pub bytes: Vec<u8>,
}

impl crate::FileLoader for ShapefileSource {
    type Error = geozero_shp::Error;

    const FILE_TYPE_NAME: &'static str = "WKT";

    fn from_bytes(bytes: Vec<u8>) -> Self {
        ShapefileSource { bytes }
    }

    fn load(self) -> Result<geo_features::FeatureCollection, Self::Error> {
        let mut bytes_cursor = io::Cursor::new(&self.bytes);
	let shapefile_reader = geozero_shp::Reader::new(&mut bytes_cursor)?;
        let mut geo_writer = geozero::geo_types::GeoWriter::new();
	// TODO: iter_geometries
	// TODO: iter_features
        shapefile_reader.iter_geometries(&mut geo_writer);
        let geometry = geo_writer.take_geometry().unwrap();
        Ok(geo_features::FeatureCollection::from_geometry(geometry).unwrap())
    }
}
