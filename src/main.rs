extern crate geo;
extern crate geojson;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate serde_json;

use geo::boundingbox::BoundingBox;
use graphics::{clear, Transformed};
use std::{env, error, fs, io, process, sync, thread};
use std::io::Write;
use serde_json::from_reader;
use geojson::conversion::TryInto;

#[allow(dead_code)]
mod lla_to_ecef;
mod window;

static PROGRAM_NAME: &'static str = "rgis";

const RED: graphics::types::Color = [1., 0., 0., 1.];
const WHITE: graphics::types::Color = [1., 1., 1., 1.];

fn render_line_string(geo_line_string: &geo::LineString<f64>,
                      draw_state: graphics::draw_state::DrawState,
                      transform: graphics::math::Matrix2d,
                      gl: &mut opengl_graphics::GlGraphics) {
    let graphics_line = graphics::line::Line::new(RED, 1.);

    let bbox = geo_line_string.bbox().unwrap();

    let bbox_width = bbox.xmax - bbox.xmin;
    let x_scale = window::WINDOW_SIZE_X as f64 / bbox_width;

    let bbox_height = bbox.ymax - bbox.ymin;
    let y_scale = window::WINDOW_SIZE_Y as f64 / bbox_height;

    let scale = x_scale.min(y_scale);

    //let transform = transform.trans(-bbox.xmin, -bbox.ymin);
    let transform = transform.flip_v();
    //let transform = transform.scale(x_scale, y_scale);

    let points = geo_line_string.0
        .iter()
        .map(|point| point.0)
        .map(|coord| {
                 geo::Coordinate {
                     x: coord.x - bbox.xmin,
                     y: coord.y - bbox.ymax,
                 }
             })
        .map(|coord| {
                 geo::Coordinate {
                     x: coord.x * scale,
                     y: coord.y * scale,
                 }
             })
        .map(|coord| [coord.x, coord.y])
        .collect::<Vec<_>>();

    for x in points.windows(2) {
        graphics_line.draw([x[0][0], x[0][1], x[1][0], x[1][1]],
                           &draw_state,
                           transform,
                           gl);
    }
}

fn render_polygon(geo_polygon: &geo::Polygon<f64>,
                  draw_state: graphics::draw_state::DrawState,
                  transform: graphics::math::Matrix2d,
                  gl: &mut opengl_graphics::GlGraphics) {
    let graphics_polygon = graphics::polygon::Polygon::new(RED);

    let bbox = geo_polygon.bbox().unwrap();

    let bbox_width = bbox.xmax - bbox.xmin;
    let x_scale = window::WINDOW_SIZE_X as f64 / bbox_width;

    let bbox_height = bbox.ymax - bbox.ymin;
    let y_scale = window::WINDOW_SIZE_Y as f64 / bbox_height;

    let scale = x_scale.min(y_scale);

    //let transform = transform.trans(-bbox.xmin, -bbox.ymin);
    let transform = transform.flip_v();
    //let transform = transform.scale(x_scale, y_scale);

    let points = geo_polygon.exterior
        .0
        .iter()
        .map(|point| point.0)
        .map(|coord| {
                 geo::Coordinate {
                     x: coord.x - bbox.xmin,
                     y: coord.y - bbox.ymax,
                 }
             })
        .map(|coord| {
                 geo::Coordinate {
                     x: coord.x * scale,
                     y: coord.y * scale,
                 }
             })
        .map(|coord| [coord.x, coord.y])
        .collect::<Vec<_>>();

    graphics_polygon.draw(&points, &draw_state, transform, gl);
}

fn rgis() -> Result<(), Box<error::Error>> {
    let mut args = env::args().skip(1);

    let geojson_file_path = match args.next() {
        Some(a) => a,
        None => return Err("usage: rgis <geojson file name>".into()),
    };

    let layers: sync::Arc<sync::RwLock<Vec<geojson::Value>>> = sync::Arc::new(sync::RwLock::new(vec![]));
    let lol = layers.clone();

    // Start a file loading thread
    let (tx, rx) = sync::mpsc::channel();
    thread::spawn(move || {
        while let Ok(geojson_file_path) = rx.recv() {
            let geojson_file = fs::File::open(geojson_file_path).expect("TODO");
            let geojson_polygon: geojson::GeoJson = from_reader(geojson_file).unwrap();
            let geojson_polygon = match geojson_polygon {
                geojson::GeoJson::Geometry(g) => g,
                _ => unreachable!(),
            };
            (&mut lol.write().unwrap()).push(geojson_polygon.value);
        }
        writeln!(io::stderr(), "File loader thread died!").expect("could not write to stderr");
    });

    tx.send(geojson_file_path).expect("TODO");

    window::window_loop(|ctx, g| {
        clear(WHITE, g);
        for geojson_value in &*layers.read().unwrap() {
            if let Some(geo_polygon) =
                geojson_value
                    .clone()
                    .try_into()
                    .ok() as Option<geo::Polygon<f64>> {
                render_polygon(&geo_polygon, ctx.draw_state, ctx.transform, g);
            } else if let Some(geo_line_string) =
                geojson_value.clone().try_into().ok() as Option<geo::LineString<f64>> {
                render_line_string(&geo_line_string, ctx.draw_state, ctx.transform, g);
            }
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
