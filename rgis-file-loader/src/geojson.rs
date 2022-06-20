#[cfg(not(target_arch = "wasm32"))]
use std::path;
use std::{io, iter};

pub enum GeoJsonSource {
    #[cfg(not(target_arch = "wasm32"))]
    Path(std::path::PathBuf),
    Bytes(Vec<u8>),
}

impl GeoJsonSource {
    pub fn load(self) -> Result<geo::Geometry<f64>, LoadGeoJsonError> {
        Ok(match self {
            #[cfg(not(target_arch = "wasm32"))]
            GeoJsonSource::Path(path) => load_from_path(&path)?,
            GeoJsonSource::Bytes(bytes) => load_from_reader(io::Cursor::new(bytes))?,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn load_from_path(geojson_file_path: &path::Path) -> Result<geo::Geometry<f64>, LoadGeoJsonError> {
    use std::fs;
    let tl = time_logger::start!("Opening file: {:?}", geojson_file_path);
    let reader = io::BufReader::new(fs::File::open(&geojson_file_path)?);
    tl.finish();

    load_from_reader(reader)
}

#[derive(thiserror::Error, Debug)]
pub enum LoadGeoJsonError {
    #[error("{0}")]
    GeoJson(#[from] geojson::Error),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
}

fn attempt_to_load_with_feature_iterator<R: io::Read>(
    iter: iter::Peekable<geojson::FeatureIterator<R>>,
) -> Result<geo::GeometryCollection<f64>, LoadGeoJsonError> {
    let tl = time_logger::start!("Parsing file and converting to geo-types");
    let mut geo_geometry_collection: geo::GeometryCollection<f64>;
    geo_geometry_collection = geo::GeometryCollection::default();
    for feature_result in iter {
        let feature = feature_result?;
        if let Some(geometry) = feature.geometry {
            geo_geometry_collection.0.push(geometry.try_into()?);
        }
    }
    tl.finish();
    Ok(geo_geometry_collection)
}

fn load_from_reader<R: io::Read + io::Seek>(
    mut reader: R,
) -> Result<geo::Geometry<f64>, LoadGeoJsonError> {
    let mut iter = geojson::FeatureIterator::new(&mut reader).peekable();

    let mut geo_geometry_collection: Option<geo::GeometryCollection<f64>> = None;

    if iter.peek().is_some() {
        if let Ok(g) = attempt_to_load_with_feature_iterator(iter) {
            geo_geometry_collection = Some(g);
        } else {
            reader.rewind()?;
        }
    }

    let geo_geometry_collection = match geo_geometry_collection {
        Some(g) => g,
        None => {
            let tl = time_logger::start!("Parsing file");
            let geojson: geojson::GeoJson = serde_json::from_reader(reader)?;
            tl.finish();

            let tl = time_logger::start!("Converting to geo-types");
            let geo_geometry_collection = geojson::quick_collection(&geojson)?;
            tl.finish();
            geo_geometry_collection
        }
    };

    Ok(geo::Geometry::GeometryCollection(geo_geometry_collection))
}
