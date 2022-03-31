#![warn(clippy::unwrap_used, clippy::expect_used)]

use clap::{Arg, Command};
use std::path;

static DEFAULT_SOURCE_SRS: &str = "EPSG:4326";
static DEFAULT_MSAA: &str = "4";

type MsaaSampleCount = u32;

#[derive(Clone)]
pub struct Values {
    pub msaa_sample_count: MsaaSampleCount,
    pub geojson_files: Vec<path::PathBuf>,
    pub source_crs: String,
}

pub fn run() -> Values {
    let matches = Command::new("rgis")
        .version("0.1.0")
        .author("Corey Farwell <coreyf@rwell.org>")
        .about("Geospatial data viewer written in Rust")
        .arg(
            Arg::new("MSAA SAMPLE COUNT")
                .long("--msaa-sample-count")
                .default_value(DEFAULT_MSAA)
                .help("Multi-Sample Anti-Aliasing sample count. Setting the sample count higher will result in smoother edges, but it will also increase the cost to render those edges. The range should generally be somewhere between 1 (no multi sampling, but cheap) to 8 (crisp but expensive).")
                .validator(|s| {
                    if s.parse::<MsaaSampleCount>().is_ok() {
                        Ok(())
                    } else {
                        Err("should be a non-zero positive integer".to_string())
                    }
                })
                .takes_value(true),
        )
        .arg(
            Arg::new("SOURCE SRS")
                .long("--source-srs")
                .default_value(DEFAULT_SOURCE_SRS)
                .help("SRS of input files")
                .takes_value(true),
        )
        .arg(Arg::new("GEOJSON FILE").multiple_occurrences(true))
        .get_matches();

    Values {
        geojson_files: matches
            .values_of("GEOJSON FILE")
            .unwrap_or_default()
            .map(|s| path::PathBuf::from(s.to_owned()))
            .collect(),
        source_crs: matches.value_of("SOURCE SRS").unwrap().to_owned(),
        msaa_sample_count: matches
            .value_of("MSAA SAMPLE COUNT")
            .unwrap()
            .parse()
            .unwrap(),
    }
}

pub struct Plugin(pub Values);

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(self.0.clone());
    }
}
