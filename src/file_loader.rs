use geojson::conversion::TryInto;
use crate::Layers;
use std::io::Write;
use std::{fs, io, sync, thread};

pub struct Thread {
    _join_handle: thread::JoinHandle<()>,
    tx: sync::mpsc::Sender<String>,
}

impl Thread {
    pub fn spawn(layers: Layers) -> Thread {
        let (tx, rx) = sync::mpsc::channel();
        let join_handle = thread::spawn(move || {
            while let Ok(geojson_file_path) = rx.recv() {
                let geojson_file = fs::File::open(geojson_file_path).expect("TODO");
                let geojson_polygon: geojson::GeoJson =
                    serde_json::from_reader(&geojson_file).unwrap();
                let geojson_polygon = match geojson_polygon {
                    geojson::GeoJson::Geometry(g) => g,
                    _ => unreachable!(),
                };
                if let Some(geo_polygon) =
                    geojson_polygon.value.clone().try_into().ok() as Option<geo::Polygon<f64>>
                {
                    (&mut layers.write().unwrap()).push(Box::new(geo_polygon));
                } else if let Some(geo_line_string) =
                    geojson_polygon.value.clone().try_into().ok() as Option<geo::LineString<f64>>
                {
                    (&mut layers.write().unwrap()).push(Box::new(geo_line_string));
                }
            }
            writeln!(io::stderr(), "File loader thread died!").expect("could not write to stderr");
        });
        Thread {
            _join_handle: join_handle,
            tx: tx,
        }
    }

    pub fn load(&self, path: String) {
        self.tx.send(path).expect("TODO");
    }
}
