use clap::{App, Arg};
use std::path;

static DEFAULT_SOURCE_SRS: &str = "EPSG:4326";
static DEFAULT_TARGET_SRS: &str = "EPSG:3857";
static DEFAULT_MSAA: &str = "4";

type MsaaSampleCount = u32;

#[derive(Clone)]
pub struct Values {
    pub msaa_sample_count: MsaaSampleCount,
    pub geojson_files: Vec<path::PathBuf>,
    pub source_srs: String,
    pub target_srs: String,
}

pub fn run() -> Values {
    let matches = App::new("rgis")
        .version("0.1.0")
        .author("Corey Farwell <coreyf@rwell.org>")
        .about("Geospatial data viewer written in Rust")
        .arg(
            Arg::with_name("MSAA SAMPLE COUNT")
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
            Arg::with_name("SOURCE SRS")
                .long("--source-srs")
                .default_value(DEFAULT_SOURCE_SRS)
                .help("SRS of input files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("TARGET SRS")
                .long("--target-srs")
                .default_value(DEFAULT_TARGET_SRS)
                .help("Reproject to this SRS")
                .takes_value(true),
        )
        .arg(Arg::with_name("GEOJSON FILE").multiple(true).required(true))
        .get_matches();

    Values {
        geojson_files: matches
            .values_of("GEOJSON FILE")
            .unwrap()
            .map(|s| path::PathBuf::from(s.to_owned()))
            .collect(),
        source_srs: matches.value_of("SOURCE SRS").unwrap().to_owned(),
        target_srs: matches.value_of("TARGET SRS").unwrap().to_owned(),
        msaa_sample_count: matches
            .value_of("MSAA SAMPLE COUNT")
            .unwrap()
            .parse()
            .unwrap(),
    }
}

pub struct Plugin(pub Values);

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::AppBuilder) {
        app.insert_resource(self.0.clone());
    }
}
