use crate::layer::Layers;
use std::convert::TryInto;
use std::fs;
use std::sync;

pub fn load(geojson_file_path: String, layers: sync::Arc<sync::RwLock<Layers>>) {
    log::info!("Opening file: {:?}", geojson_file_path);
    let geojson_file = fs::File::open(&geojson_file_path).expect("TODO");
    log::info!("Parsing file: {:?}", geojson_file_path);
    let geojson: geojson::GeoJson = serde_json::from_reader(&geojson_file).unwrap();
    match geojson {
        geojson::GeoJson::Geometry(g) => load_geojson_geometry(g, layers),
        geojson::GeoJson::Feature(f) => load_geojson_feature(f, layers),
        geojson::GeoJson::FeatureCollection(f) => {
            for feature in f.features {
                load_geojson_feature(feature, layers.clone())
            }
        }
    };
}

fn load_geojson_feature(
    geojson_feature: geojson::Feature,
    layers: sync::Arc<sync::RwLock<Layers>>,
) {
    if let Some(geometry) = geojson_feature.geometry {
        load_geojson_geometry(geometry, layers)
    }
}

fn load_geojson_geometry(
    geojson_geometry: geojson::Geometry,
    layers: sync::Arc<sync::RwLock<Layers>>,
) {
    let geojson_value = geojson_geometry.value;

    let mut l = layers.write().unwrap();

    match geojson_value {
        g @ geojson::Value::LineString(_) => {
            let g = (g.try_into().ok() as Option<geo::LineString<f64>>).unwrap();
            l.add(geo::Geometry::LineString(g));
        }
        g @ geojson::Value::Polygon(_) => {
            let g = (g.try_into().ok() as Option<geo::Polygon<f64>>).unwrap();
            l.add(geo::Geometry::Polygon(g));
        }
        g @ geojson::Value::MultiLineString(_) => {
            let g = (g.try_into().ok() as Option<geo::MultiLineString<f64>>).unwrap();
            l.add(geo::Geometry::MultiLineString(g));
        }
        g @ geojson::Value::MultiPolygon(_) => {
            let g = (g.try_into().ok() as Option<geo::MultiPolygon<f64>>).unwrap();
            l.add(geo::Geometry::MultiPolygon(g));
        }
        geojson::Value::GeometryCollection(g) => {
            for geojson_geometry in g {
                load_geojson_geometry(geojson_geometry, layers.clone());
            }
        }
        _ => {}
    }
}
