use std::io::Write;
use std::{error, io, process, sync};

mod canvas;
mod event_loop;
mod window;

static PROGRAM_NAME: &str = "rgis";

static SHOW_DEBUG_UI: bool = false; // TODO: Make this a CLI flag

// TODO: allow these to be controller at command line
static SOURCE_PROJECTION: &str = "EPSG:4326";
static TARGET_PROJECTION: &str = "EPSG:3857";

fn bg_color() -> pathfinder_color::ColorF {
    pathfinder_color::ColorF::white()
}

fn rgis() -> Result<(), Box<dyn error::Error>> {
    let geojson_file_paths = rgis_cli::run()?;

    let layers = sync::Arc::new(sync::RwLock::new(rgis_layers::Layers::new()));

    let window = window::Window::new(layers.clone());

    for geojson_file_path in geojson_file_paths {
        let event_loop_proxy = window.event_loop.create_proxy();
        rgis_file_loader::load(
            geojson_file_path,
            layers.clone(),
            SOURCE_PROJECTION,
            TARGET_PROJECTION,
            move || {
                event_loop_proxy
                    .send_event(window::UserEvent::LayerAdded)
                    .unwrap()
            },
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
