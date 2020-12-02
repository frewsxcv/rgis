use bevy::{prelude::*, render::pass::ClearColor};
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

fn load_layers_from_cli(mut layers: ResMut<rgis_layers::Layers>) {
    let geojson_file_paths = rgis_cli::run().unwrap();

    for geojson_file_path in geojson_file_paths {
        // let event_loop_proxy = window.event_loop.create_proxy();
        let count = rgis_file_loader::load(
            geojson_file_path,
            &mut layers,
            SOURCE_PROJECTION,
            TARGET_PROJECTION,
        );
        if count > 0 {

        }
    }
}

fn main() {
    env_logger::init();

    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(rgis_layers::Layers::new())
        .add_startup_system(load_layers_from_cli.system())
        .add_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .run();

    // if let Err(e) = rgis() {
    //     writeln!(io::stderr(), "{}: {}", PROGRAM_NAME, e).expect("could not write to stderr");
    //     process::exit(1);
    // }
}
