use crate::layer::{Layer, Layers};
use geo::bounding_rect::BoundingRect;
use std::convert::TryInto;
use std::io::Write;
use std::{fs, io, sync, thread};

pub struct Thread {
    _join_handle: thread::JoinHandle<()>,
    tx: sync::mpsc::Sender<String>,
}

impl Thread {
    pub fn spawn(mut layers: Layers) -> Thread {
        let (tx, rx) = sync::mpsc::channel();
        let join_handle = thread::spawn(move || {
            while let Ok(geojson_file_path) = rx.recv() {
                println!("received: {:?}", geojson_file_path);
                let geojson_file = fs::File::open(geojson_file_path).expect("TODO");
                println!("opened");
                let geojson: geojson::GeoJson =
                    serde_json::from_reader(&geojson_file).unwrap();
                println!("parsed");
                match geojson {
                    geojson::GeoJson::Geometry(g) => Thread::load_geojson_geometry(&mut layers, g),
                    geojson::GeoJson::Feature(f) => Thread::load_geojson_feature(&mut layers, f),
                    geojson::GeoJson::FeatureCollection(f) => {
                        for feature in f.features {
                            Thread::load_geojson_feature(&mut layers, feature)
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

    fn load_geojson_feature(layers: &mut Layers, geojson_feature: geojson::Feature) {
        println!("feature loading");
        if let Some(geometry) = geojson_feature.geometry {
            Thread::load_geojson_geometry(layers, geometry)
        }
    }

    fn load_geojson_geometry(layers: &mut Layers, geojson_geometry: geojson::Geometry) {
        let geojson_value = geojson_geometry.value;
        if let geojson::Value::GeometryCollection(geojson_geometry_collection) = geojson_value {
            for geojson_geometry in geojson_geometry_collection {
                Thread::load_geojson_geometry(layers, geojson_geometry);
            }
            return;
        }

        if let Some(geo_polygon) =
            geojson_value.clone().try_into().ok() as Option<geo::Polygon<f64>>
        {
            (&mut layers.write().unwrap()).push(Layer {
                bounding_rect: geo_polygon.bounding_rect().unwrap(),
                geometry: geo::Geometry::Polygon(geo_polygon),
            });
        } else if let Some(geo_line_string) =
            geojson_value.clone().try_into().ok() as Option<geo::LineString<f64>>
        {
            (&mut layers.write().unwrap()).push(Layer {
                bounding_rect: geo_line_string.bounding_rect().unwrap(),
                geometry: geo::Geometry::LineString(geo_line_string),
            });
        }
    }

    pub fn load(&self, path: String) {
        println!("loading: {:?}", path);
        self.tx.send(path).expect("TODO");
    }
}