use crate::renderable::Renderable;
use std::io::Write;
use std::{error, io, process, sync};

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

    window::build_window(|canvas| {
        loop {
            println!("rerendering");

            ::std::thread::sleep(::std::time::Duration::from_secs(5));

            // if tmp.len() > 80 {
            if layer::LAYERS.read().unwrap().len() > 0 {
                render(canvas);
                break;
            }

            ::std::thread::sleep(::std::time::Duration::from_secs(1));
        }
    });

    Ok(())
}

fn render(canvas: &mut pathfinder_canvas::CanvasRenderingContext2D) {
    let layers = &layer::LAYERS.read().unwrap();

    let b = bbox_many(&layers[..]);

    for layer in &layers[..] {
        layer.geometry.to_owned().render(canvas, b);
    }
}

fn bbox_many(geometries: &[layer::Layer]) -> geo::Rect<f64> {
    let mut iter = geometries.into_iter();
    let r = iter.next().unwrap().bounding_rect;
    let mut min_x = r.min().x;
    let mut min_y = r.min().y;
    let mut max_x = r.max().x;
    let mut max_y = r.max().y;
    for g in iter {
        let b = g.bounding_rect;
        if b.min().x < min_x {
            min_x = b.min().x;
        }
        if b.min().y < min_y {
            min_y = b.min().y;
        }
        if b.max().x > max_x {
            max_x = b.max().x;
        }
        if b.max().y > max_y {
            max_y = b.max().y;
        }
    }
    geo::Rect::new(
        geo::Coordinate { x: min_x, y: min_y },
        geo::Coordinate { x: max_x, y: max_y },
    )
}

fn main() {
    if let Err(e) = rgis() {
        writeln!(io::stderr(), "{}: {}", PROGRAM_NAME, e).expect("could not write to stderr");
        process::exit(1);
    }
}
