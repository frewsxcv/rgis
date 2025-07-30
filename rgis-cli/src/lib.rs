use bevy::prelude::*;
use clap::Command;

pub fn run() -> Result<(), String> {
    Command::new("rgis")
        .author("Corey Farwell <coreyf@rwell.org>")
        .about("Geospatial data viewer written in Rust")
        .get_matches();

    Ok(())
}
