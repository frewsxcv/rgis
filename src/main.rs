use geojson::conversion::TryInto;
use std::io::Write;
use std::{env, error, fs, io, process, sync, thread};

#[allow(dead_code)]
mod lla_to_ecef;
mod renderable;
mod window;

static PROGRAM_NAME: &'static str = "rgis";

struct FileLoadingThread {
    _join_handle: thread::JoinHandle<()>,
    tx: sync::mpsc::Sender<String>,
}

type Layers = sync::Arc<sync::RwLock<Vec<Box<dyn renderable::Renderable>>>>;

impl FileLoadingThread {
    fn spawn(layers: Layers) -> FileLoadingThread {
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
        FileLoadingThread {
            _join_handle: join_handle,
            tx: tx,
        }
    }

    fn load(&self, path: String) {
        self.tx.send(path).expect("TODO");
    }
}

fn rgis() -> Result<(), Box<dyn error::Error>> {
    let mut args = env::args().skip(1);

    let geojson_file_path = match args.next() {
        Some(a) => a,
        None => return Err("usage: rgis <geojson file name>".into()),
    };

    let layers = sync::Arc::new(sync::RwLock::new(vec![]));

    let file_loading_thread = FileLoadingThread::spawn(layers.clone());
    file_loading_thread.load(geojson_file_path);


    window::build_window(|canvas| {
        for renderable in &*layers.read().unwrap() {
            renderable.render(canvas);
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
