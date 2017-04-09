extern crate geo;
extern crate geojson;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate serde_json;
extern crate spade;

use graphics::clear;
use std::{env, error, fs, io, process, sync, thread};
use std::io::Write;
use serde_json::from_reader;
use geojson::conversion::TryInto;

#[allow(dead_code)]
mod lla_to_ecef;
mod renderable;
mod window;

static PROGRAM_NAME: &'static str = "rgis";

const RED: graphics::types::Color = [1., 0., 0., 1.];
const WHITE: graphics::types::Color = [1., 1., 1., 1.];
const BLACK: graphics::types::Color = [0., 0., 0., 1.];

struct FileLoadingThread {
    _join_handle: thread::JoinHandle<()>,
    tx: sync::mpsc::Sender<String>,
}

type Layers = sync::Arc<sync::RwLock<Vec<Box<renderable::Renderable>>>>;

impl FileLoadingThread {
    fn spawn(layers: Layers) -> FileLoadingThread {
        let (tx, rx) = sync::mpsc::channel();
        let join_handle = thread::spawn(move || {
            while let Ok(geojson_file_path) = rx.recv() {
                let geojson_file = fs::File::open(geojson_file_path).expect("TODO");
                let geojson_polygon: geojson::GeoJson = from_reader(geojson_file).unwrap();
                let geojson_polygon = match geojson_polygon {
                    geojson::GeoJson::Geometry(g) => g,
                    _ => unreachable!(),
                };
                if let Some(geo_polygon) =
                    geojson_polygon.value.clone().try_into().ok() as Option<geo::Polygon<f64>> {
                    (&mut layers.write().unwrap()).push(Box::new(geo_polygon));
                } else if let Some(geo_line_string) =
                    geojson_polygon.value.clone().try_into().ok() as Option<geo::LineString<f64>> {
                    (&mut layers.write().unwrap()).push(Box::new(geo_line_string));
                }
            }
            writeln!(io::stderr(), "File loader thread died!").expect("could not write to stderr");
        });
        FileLoadingThread {
            _join_handle: join_handle,
            tx: tx,
        }
    }

    fn load(&self, path: String) {
        self.tx.send(path).expect("TODO");
    }
}

fn rgis() -> Result<(), Box<error::Error>> {
    let mut args = env::args().skip(1);

    let geojson_file_path = match args.next() {
        Some(a) => a,
        None => return Err("usage: rgis <geojson file name>".into()),
    };

    let layers = sync::Arc::new(sync::RwLock::new(vec![]));

    let file_loading_thread = FileLoadingThread::spawn(layers.clone());
    file_loading_thread.load(geojson_file_path);

    // Start a file loading thread

    window::window_loop(|ctx, g| {
                            clear(WHITE, g);
                            for renderable in &*layers.read().unwrap() {
                                renderable.render(ctx.draw_state, ctx.transform, g);
                            }
                        });

    Ok(())
}

fn main() {
    if let Err(e) = rgis() {
        writeln!(io::stderr(), "{}: {}", PROGRAM_NAME, e).expect("could not write to stderr");
        process::exit(1);
    }
}
