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

    let file_loading_thread = file_loader::Thread::spawn();

    for geojson_file_path in geojson_file_paths {
        file_loading_thread.load(geojson_file_path);
    }

    window::build_window();

    Ok(())
}

fn render(
    window_size: pathfinder_geometry::vector::Vector2I,
) -> pathfinder_canvas::CanvasRenderingContext2D {
    let font_context = CanvasFontContext::from_system_source();
    let mut canvas = Canvas::new(window_size.to_f32()).get_context_2d(font_context);

    let layers = layer::LAYERS.0.read().unwrap();

    let b = bbox_many(&layers[..]);

    for layer in &layers[..] {
        layer.geometry.render(&mut canvas, b, layer.color);
    }

    canvas
}

fn bbox_merge(a: geo::Rect<f64>, b: geo::Rect<f64>) -> geo::Rect<f64> {
    geo::Rect::new(
        geo::Coordinate {
            x: a.min().x.min(b.min().x),
            y: a.min().y.min(b.min().y),
        },
        geo::Coordinate {
            x: a.max().x.max(b.max().x),
            y: a.max().y.max(b.max().y),
        },
    )
}

fn bbox_many(geometries: &[layer::Layer]) -> geo::Rect<f64> {
    let mut iter = geometries.into_iter();
    let first_rect = iter.next().unwrap().bounding_rect;
    iter.fold(first_rect, |acc, next| {
        bbox_merge(acc, next.bounding_rect)
    })
}

fn main() {
    env_logger::init();

    if let Err(e) = rgis() {
        writeln!(io::stderr(), "{}: {}", PROGRAM_NAME, e).expect("could not write to stderr");
        process::exit(1);
    }
}
