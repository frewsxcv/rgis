use crate::render::Render;
use pathfinder_canvas::{Canvas, CanvasFontContext};
use std::io::Write;
use std::{error, io, process};

mod cli;
mod color;
mod file_loader;
mod layer;
#[allow(dead_code)]
mod lla_to_ecef;
mod render;
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
    log::info!("Creating a canvas");
    let mut canvas = new_canvas(window_size);
    let layers = layer::LAYERS.data.read().unwrap();
    let bounding_rect = layer::LAYERS.bounding_rect.read().unwrap();

    for layer in &layers[..] {
        layer.geometry.render(crate::render::RenderContext {
            canvas: &mut canvas,
            extent: bounding_rect.unwrap(),
            color: layer.color,
            window_size: window_size,
        })
    }

    canvas
}

fn new_canvas(
    window_size: pathfinder_geometry::vector::Vector2I,
) -> pathfinder_canvas::CanvasRenderingContext2D {
    let font_context = CanvasFontContext::from_system_source();
    Canvas::new(window_size.to_f32()).get_context_2d(font_context)
}

fn main() {
    env_logger::init();

    if let Err(e) = rgis() {
        writeln!(io::stderr(), "{}: {}", PROGRAM_NAME, e).expect("could not write to stderr");
        process::exit(1);
    }
}
