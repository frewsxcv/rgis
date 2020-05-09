use std::io::Write;
use std::{error, io, process, sync};

mod canvas;
mod cli;
mod color;
mod event_loop;
mod file_loader;
mod layer;
#[allow(dead_code)]
mod lla_to_ecef;
mod window;

static PROGRAM_NAME: &'static str = "rgis";

fn rgis() -> Result<(), Box<dyn error::Error>> {
    let geojson_file_paths = cli::run()?;

    let layers = sync::Arc::new(sync::RwLock::new(layer::Layers::new()));

    let window = window::Window::new(layers.clone());

    for geojson_file_path in geojson_file_paths {
        file_loader::load(
            geojson_file_path,
            window.event_loop.create_proxy(),
            layers.clone(),
        );
    }

    window.start_event_loop();
}

fn main() {
    env_logger::init();

    if let Err(e) = rgis() {
        writeln!(io::stderr(), "{}: {}", PROGRAM_NAME, e).expect("could not write to stderr");
        process::exit(1);
    }
}
