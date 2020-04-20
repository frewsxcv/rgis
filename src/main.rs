use std::io::Write;
use std::{error, io, process, sync};

mod cli;
mod file_loader;
#[allow(dead_code)]
mod lla_to_ecef;
mod renderable;
mod window;

static PROGRAM_NAME: &'static str = "rgis";

type Layers = sync::Arc<sync::RwLock<Vec<Box<dyn renderable::Renderable>>>>;

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
            if tmp.len() > 0 {
                for renderable in tmp {
                    renderable.render(canvas);
                }
                break;
            }

            ::std::thread::sleep(::std::time::Duration::from_secs(1));
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
