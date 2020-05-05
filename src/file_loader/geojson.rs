use crate::layer::Layers;
use std::convert::TryInto;
use std::fs;
use std::sync;

pub fn load(geojson_file_path: String, layers: sync::Arc<sync::RwLock<Layers>>) -> usize {
    log::info!("Opening file: {:?}", geojson_file_path);
    let geojson_file = fs::File::open(&geojson_file_path).expect("TODO");
    log::info!("Parsing file: {:?}", geojson_file_path);
    let geojson: geojson::GeoJson = serde_json::from_reader(&geojson_file).unwrap();
    log::info!("Parsed file: {:?}", geojson_file_path);
    let count = match geojson {
        geojson::GeoJson::Geometry(g) => load_geojson_geometry(g, layers),
        geojson::GeoJson::Feature(f) => load_geojson_feature(f, layers),
        geojson::GeoJson::FeatureCollection(f) => {
            let mut count = 0;
            for feature in f.features {
                count += load_geojson_feature(feature, layers.clone())
            }
            count
        }
    };
    log::info!("Loaded file: {:?}", geojson_file_path);
    count
}

fn load_geojson_feature(
    geojson_feature: geojson::Feature,
    layers: sync::Arc<sync::RwLock<Layers>>,
) -> usize {
    if let Some(geometry) = geojson_feature.geometry {
        load_geojson_geometry(geometry, layers)
    } else {
        0
    }
}

fn load_geojson_geometry(
    geojson_geometry: geojson::Geometry,
    layers: sync::Arc<sync::RwLock<Layers>>,
) -> usize {
    let geojson_value = geojson_geometry.value;

    let mut l = layers.write().unwrap();

    match geojson_value {
        g @ geojson::Value::LineString(_) => {
            let g = (g.try_into().ok() as Option<geo::LineString<f64>>).unwrap();
            l.add(geo::Geometry::LineString(g));
            1
        }
        g @ geojson::Value::Polygon(_) => {
            let g = (g.try_into().ok() as Option<geo::Polygon<f64>>).unwrap();
            l.add(geo::Geometry::Polygon(g));
            1
        }
        g @ geojson::Value::MultiLineString(_) => {
            let g = (g.try_into().ok() as Option<geo::MultiLineString<f64>>).unwrap();
            l.add(geo::Geometry::MultiLineString(g));
            1
        }
        g @ geojson::Value::MultiPolygon(_) => {
            let g = (g.try_into().ok() as Option<geo::MultiPolygon<f64>>).unwrap();
            l.add(geo::Geometry::MultiPolygon(g));
            1
        }
        geojson::Value::GeometryCollection(g) => {
            let mut count = 0;
            for geojson_geometry in g {
                count += load_geojson_geometry(geojson_geometry, layers.clone());
            }
            count
        }
        _ => 0,
    }
}
