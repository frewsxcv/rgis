use bevy::prelude::*;
use clap::{Arg, Command};

pub struct CliArgs {
    pub url: Option<String>,
}

pub fn run() -> Result<CliArgs, String> {
    let matches = Command::new("rgis")
        .author("Corey Farwell <coreyf@rwell.org>")
        .about("Geospatial data viewer written in Rust")
        .arg(
            Arg::new("url")
                .long("url")
                .help("URL to a GeoJSON file to load on startup")
                .value_name("URL"),
        )
        .get_matches();

    Ok(CliArgs {
        url: matches.get_one::<String>("url").cloned(),
    })
}
