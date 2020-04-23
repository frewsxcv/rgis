use crate::layer::{Layer, LAYERS};
use geo::bounding_rect::BoundingRect;
use std::convert::TryInto;
use std::io::Write;
use std::{fs, io, sync, thread};
use crate::renderable::next_color;

pub struct Thread {
    _join_handle: thread::JoinHandle<()>,
    tx: sync::mpsc::Sender<String>,
}

impl Thread {
    pub fn spawn() -> Thread {
        let (tx, rx) = sync::mpsc::channel();
        let join_handle = thread::spawn(move || {
            while let Ok(geojson_file_path) = rx.recv() {
                log::info!("Opening file: {:?}", geojson_file_path);
                let geojson_file = fs::File::open(&geojson_file_path).expect("TODO");
                log::info!("Parsing file: {:?}", geojson_file_path);
                let geojson: geojson::GeoJson = serde_json::from_reader(&geojson_file).unwrap();
                match geojson {
                    geojson::GeoJson::Geometry(g) => Thread::load_geojson_geometry(g),
                    geojson::GeoJson::Feature(f) => Thread::load_geojson_feature(f),
                    geojson::GeoJson::FeatureCollection(f) => {
                        for feature in f.features {
                            Thread::load_geojson_feature(feature)
                        }
                    }
                };
            }
            writeln!(io::stderr(), "File loader thread died!").expect("could not write to stderr");
        });
        Thread {
            _join_handle: join_handle,
            tx: tx,
        }
    }

    fn load_geojson_feature(geojson_feature: geojson::Feature) {
        if let Some(geometry) = geojson_feature.geometry {
            Thread::load_geojson_geometry(geometry)
        }
    }

    fn load_geojson_geometry(geojson_geometry: geojson::Geometry) {
        let geojson_value = geojson_geometry.value;

        match geojson_value {
            g @ geojson::Value::LineString(_) => {
                let g = (g.try_into().ok() as Option<geo::LineString<f64>>).unwrap();
                (&mut LAYERS.write().unwrap()).push(Layer {
                    bounding_rect: g.bounding_rect().unwrap(),
                    geometry: geo::Geometry::LineString(g),
                    color: next_color(),
                });
            },
            g @ geojson::Value::Polygon(_) => {
                let g = (g.try_into().ok() as Option<geo::Polygon<f64>>).unwrap();
                (&mut LAYERS.write().unwrap()).push(Layer {
                    bounding_rect: g.bounding_rect().unwrap(),
                    geometry: geo::Geometry::Polygon(g),
                    color: next_color(),
                });
            },
            g @ geojson::Value::MultiLineString(_) => {
                let g = (g.try_into().ok() as Option<geo::MultiLineString<f64>>).unwrap();
                (&mut LAYERS.write().unwrap()).push(Layer {
                    bounding_rect: g.bounding_rect().unwrap(),
                    geometry: geo::Geometry::MultiLineString(g),
                    color: next_color(),
                });
            },
            g @ geojson::Value::MultiPolygon(_) => {
                let g = (g.try_into().ok() as Option<geo::MultiPolygon<f64>>).unwrap();
                (&mut LAYERS.write().unwrap()).push(Layer {
                    bounding_rect: g.bounding_rect().unwrap(),
                    geometry: geo::Geometry::MultiPolygon(g),
                    color: next_color(),
                });
            },
            geojson::Value::GeometryCollection(g) => {
                for geojson_geometry in g {
                    Thread::load_geojson_geometry(geojson_geometry);
                }
            }
            _ => {}
        }
    }

    pub fn load(&self, path: String) {
        self.tx.send(path).expect("TODO");
    }
}
