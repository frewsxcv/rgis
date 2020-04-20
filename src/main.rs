use std::io::Write;
use std::{error, io, process, sync};
use geo::bounding_rect::BoundingRect;
use crate::renderable::Renderable;

mod cli;
mod file_loader;
#[allow(dead_code)]
mod lla_to_ecef;
mod renderable;
mod window;

static PROGRAM_NAME: &'static str = "rgis";

type Layers = sync::Arc<sync::RwLock<Vec<geo::Geometry<f64>>>>;

fn rgis() -> Result<(), Box<dyn error::Error>> {
    let geojson_file_paths = cli::run()?;

    let layers = sync::Arc::new(sync::RwLock::new(vec![]));

    let file_loading_thread = file_loader::Thread::spawn(layers.clone());

    for geojson_file_path in geojson_file_paths {
        file_loading_thread.load(geojson_file_path);
    }

    window::build_window(|canvas| {
        loop {
            println!("rerendering");

            let tmp = &*layers.read().unwrap();
            // if tmp.len() > 80 {
            if tmp.len() > 0 {
                let b = bbox_many(tmp);

                for renderable in tmp {
                    renderable.to_owned().render(canvas, b);
                }
                break;
            }

            ::std::thread::sleep(::std::time::Duration::from_secs(1));
        }
    });

    Ok(())
}

fn bbox_many(geometries: &[geo::Geometry<f64>]) -> geo::Rect<f64> {
    let mut iter = geometries.into_iter();
    let r = bbox_one(iter.next().unwrap());
    let mut min_x = r.min().x;
    let mut min_y = r.min().y;
    let mut max_x = r.max().x;
    let mut max_y = r.max().y;
    for g in iter {
        let b = bbox_one(g);
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

fn bbox_one(geometry: &geo::Geometry<f64>) -> geo::Rect<f64> {
    match geometry {
        geo::Geometry::Polygon(p) => p.bounding_rect().unwrap(),
        geo::Geometry::LineString(p) => p.bounding_rect().unwrap(),
        _ => unimplemented!("bbox_one not implemented for type"),
    }
}

fn main() {
    if let Err(e) = rgis() {
        writeln!(io::stderr(), "{}: {}", PROGRAM_NAME, e).expect("could not write to stderr");
        process::exit(1);
    }
}
