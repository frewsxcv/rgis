use std::io::Write;
use std::{error, io, process, sync};

mod cli;
mod file_loader;
#[allow(dead_code)]
mod lla_to_ecef;
mod renderable;
mod window;

static PROGRAM_NAME: &'static str = "rgis";

type Layers = sync::Arc<sync::RwLock<Vec<Box<dyn renderable::Renderable>>>>;

fn rgis() -> Result<(), Box<dyn error::Error>> {
    let geojson_file_paths = cli::run()?;

    let layers = sync::Arc::new(sync::RwLock::new(vec![]));

    let file_loading_thread = file_loader::Thread::spawn(layers.clone());

    for geojson_file_path in geojson_file_paths {
        file_loading_thread.load(geojson_file_path);
    }

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
