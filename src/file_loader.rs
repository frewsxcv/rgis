use crate::layer::LAYERS;
use crate::window::UserEvent;
use glutin::event_loop::EventLoopProxy;
use std::convert::TryInto;
use std::fs;

pub fn load_file_in_thread(geojson_file_path: String, event_loop_proxy: EventLoopProxy<UserEvent>) {
    let e = event_loop_proxy.clone();
    rayon::spawn(move || {
        load_file(geojson_file_path);
        e.send_event(UserEvent::Render).unwrap();
    })
}

pub fn load_file(geojson_file_path: String) {
    log::info!("Opening file: {:?}", geojson_file_path);
    let geojson_file = fs::File::open(&geojson_file_path).expect("TODO");
    log::info!("Parsing file: {:?}", geojson_file_path);
    let geojson: geojson::GeoJson = serde_json::from_reader(&geojson_file).unwrap();
    match geojson {
        geojson::GeoJson::Geometry(g) => load_geojson_geometry(g),
        geojson::GeoJson::Feature(f) => load_geojson_feature(f),
        geojson::GeoJson::FeatureCollection(f) => {
            for feature in f.features {
                load_geojson_feature(feature)
            }
        }
    };
}

fn load_geojson_feature(geojson_feature: geojson::Feature) {
    if let Some(geometry) = geojson_feature.geometry {
        load_geojson_geometry(geometry)
    }
}

fn load_geojson_geometry(geojson_geometry: geojson::Geometry) {
    let geojson_value = geojson_geometry.value;

    match geojson_value {
        g @ geojson::Value::LineString(_) => {
            let g = (g.try_into().ok() as Option<geo::LineString<f64>>).unwrap();
            LAYERS.add(geo::Geometry::LineString(g));
        }
        g @ geojson::Value::Polygon(_) => {
            let g = (g.try_into().ok() as Option<geo::Polygon<f64>>).unwrap();
            LAYERS.add(geo::Geometry::Polygon(g));
        }
        g @ geojson::Value::MultiLineString(_) => {
            let g = (g.try_into().ok() as Option<geo::MultiLineString<f64>>).unwrap();
            LAYERS.add(geo::Geometry::MultiLineString(g));
        }
        g @ geojson::Value::MultiPolygon(_) => {
            let g = (g.try_into().ok() as Option<geo::MultiPolygon<f64>>).unwrap();
            LAYERS.add(geo::Geometry::MultiPolygon(g));
        }
        geojson::Value::GeometryCollection(g) => {
            for geojson_geometry in g {
                load_geojson_geometry(geojson_geometry);
            }
        }
        _ => {}
    }
}
