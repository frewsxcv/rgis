use std::io::Write;
use std::{error, io, process, sync};

mod canvas;
mod cli;
mod color;
mod event_loop;
mod file_loader;
mod layer;
mod window;

static PROGRAM_NAME: &str = "rgis";

static SHOW_DEBUG_UI: bool = false; // TODO: Make this a CLI flag

fn bg_color() -> rgx::color::Rgba8 {
    rgx::color::Rgba8::WHITE
}

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
