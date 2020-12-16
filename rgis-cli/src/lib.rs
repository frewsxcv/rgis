use clap::{App, Arg};

static DEFAULT_SOURCE_SRS: &str = "EPSG:4326";
static DEFAULT_TARGET_SRS: &str = "EPSG:3857";

pub struct Values {
    pub geojson_files: Vec<String>,
    pub source_srs: String,
    pub target_srs: String,
}

pub fn run() -> Values {
    let matches = App::new("rgis")
        .version("0.1.0")
        .author("Corey Farwell <coreyf@rwell.org>")
        .about("Geospatial data viewer written in Rust")
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
            .map(|s| s.to_owned())
            .collect(),
        source_srs: matches.value_of("SOURCE SRS").unwrap().to_owned(),
        target_srs: matches.value_of("SOURCE SRS").unwrap().to_owned(),
    }
}
