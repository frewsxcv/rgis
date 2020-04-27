use crate::layer::LAYERS;
use std::convert::TryInto;
use std::io::Write;
use std::{fs, io, sync, thread};

#[derive(Debug)]
pub struct Thread {
    join_handle: thread::JoinHandle<()>,
    load_tx: sync::mpsc::Sender<String>,
    pub after_load_rx: sync::mpsc::Receiver<()>,
}

impl Thread {
    pub fn spawn() -> Thread {
        let (load_tx, load_rx) = sync::mpsc::channel();
        let (after_load_tx, after_load_rx) = sync::mpsc::channel();

        let join_handle = thread::spawn(move || {
            while let Ok(geojson_file_path) = load_rx.recv() {
                load_file(geojson_file_path);
                after_load_tx.send(()).unwrap();
            }
            writeln!(io::stderr(), "File loader thread died!").expect("could not write to stderr");
        });

        Thread {
            join_handle,
            load_tx,
            after_load_rx,
        }
    }

    pub fn load(&self, path: String) {
        self.load_tx.send(path).expect("TODO");
    }
}

fn load_file(geojson_file_path: String) {
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
