use std::env;
use std::error::Error;

pub fn run() -> Result<impl Iterator<Item = String>, Box<dyn Error>> {
    let args = env::args();

    if args.len() < 2 {
        return Err("usage: rgis <geojson file name>".into());
    }

    Ok(args.skip(1))
}
