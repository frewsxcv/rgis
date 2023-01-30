#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;
use clap::{Arg, ArgAction, Command};

static DEFAULT_MSAA: &str = "4";

type MsaaSampleCount = u32;

#[derive(Clone, Resource)]
pub struct Values {
    pub msaa_sample_count: MsaaSampleCount,
}

pub fn run() -> Result<Values, String> {
    let matches = Command::new("rgis")
        .author("Corey Farwell <coreyf@rwell.org>")
        .about("Geospatial data viewer written in Rust")
        .arg(
            Arg::new("MSAA SAMPLE COUNT")
                .long("msaa-sample-count")
                .default_value(DEFAULT_MSAA)
                .action(ArgAction::Set)
                .help("Multi-Sample Anti-Aliasing sample count. Setting the sample count higher will result in smoother edges, but it will also increase the cost to render those edges. The range should generally be somewhere between 1 (no multi sampling, but cheap) to 8 (crisp but expensive).")
                .value_parser(clap::value_parser!(u32))
        )
        .get_matches();

    Ok(Values {
        msaa_sample_count: *matches
            .get_one::<MsaaSampleCount>("MSAA SAMPLE COUNT")
            .ok_or("Could not fetch MSAA sample count from clap")?,
    })
}

pub struct Plugin(pub Values);

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(self.0.clone());
    }
}
