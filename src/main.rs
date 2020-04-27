use crate::renderable::Render;
use pathfinder_canvas::{Canvas, CanvasFontContext};
use std::io::Write;
use std::{error, io, process};

mod cli;
mod file_loader;
mod layer;
#[allow(dead_code)]
mod lla_to_ecef;
mod renderable;
mod window;

static PROGRAM_NAME: &'static str = "rgis";

fn rgis() -> Result<(), Box<dyn error::Error>> {
    let geojson_file_paths = cli::run()?;

    let window = window::Window::new();

    for geojson_file_path in geojson_file_paths {
        file_loader::load_file_in_thread(geojson_file_path, window.event_loop.create_proxy());
    }

    window.start_event_loop();
}

fn render(
    window_size: pathfinder_geometry::vector::Vector2I,
) -> pathfinder_canvas::CanvasRenderingContext2D {
    println!("rerendering with window size: {:?}", window_size);
    let font_context = CanvasFontContext::from_system_source();
    let mut canvas = Canvas::new(window_size.to_f32()).get_context_2d(font_context);

    let layers = layer::LAYERS.data.read().unwrap();
    let bounding_rect = layer::LAYERS.bounding_rect.read().unwrap();

    println!("bounding_rect: {:?}", bounding_rect);

    for layer in &layers[..] {
        layer.geometry.render(
            &mut canvas,
            bounding_rect.unwrap(),
            layer.color,
            window_size,
        );
    }

    canvas
}

fn main() {
    env_logger::init();

    if let Err(e) = rgis() {
        writeln!(io::stderr(), "{}: {}", PROGRAM_NAME, e).expect("could not write to stderr");
        process::exit(1);
    }
}
