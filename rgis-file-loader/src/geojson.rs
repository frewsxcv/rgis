use rgis_layers::Layers;
use std::convert::TryInto;
use std::fs;
use std::io;

pub fn load(
    geojson_file_path: String,
    layers: &mut Layers,
    source_projection: &str,
    target_projection: &str,
) -> Vec<rgis_layers::LayerId> {
    let tl = time_logger::start(format!("Opening file: {:?}", geojson_file_path));
    let geojson_file = io::BufReader::new(fs::File::open(&geojson_file_path).expect("TODO"));
    tl.finish();

    let tl = time_logger::start(format!("Parsing file: {:?}", geojson_file_path));
    let geojson: geojson::GeoJson = serde_json::from_reader(geojson_file).unwrap();
    tl.finish();

    let tl = time_logger::start(format!("Converting to geo-types: {:?}", geojson_file_path));
    let geo_geometry_collection: geo_types::GeometryCollection<f64> =
        geojson::quick_collection(&geojson).unwrap();
    tl.finish();

    let tl = time_logger::start(format!("Adding new layer: {:?}", geojson_file_path));
    let layer_id = layers.add(
        geo_types::Geometry::GeometryCollection(geo_geometry_collection),
        None,
        source_projection,
        target_projection,
    );
    tl.finish();

    vec![layer_id]
    /*
    let layer_ids = match geojson {
        geojson::GeoJson::Geometry(g) => {
            log::info!("Loading GeoJSON Geometry");
            load_geojson_geometry(g, layers, None, source_projection, target_projection)
        }
        geojson::GeoJson::Feature(f) => {
            log::info!("Loading GeoJSON Feature");
            load_geojson_feature(f, layers, source_projection, target_projection)
        }
        geojson::GeoJson::FeatureCollection(f) => {
            let num_features = f.features.len();
            log::info!(
                "Loading GeoJSON FeatureCollection ({} features)",
                num_features
            );
            f.features
                .into_iter()
                .enumerate()
                .inspect(|(i, _)| {
                    if i % 300 == 0 {
                        log::info!("...Loaded {}%", *i as f32 / num_features as f32);
                    }
                })
                .flat_map(|(_, feature)| {
                    load_geojson_feature(feature, layers, source_projection, target_projection)
                })
                .collect()
        }
    };
    log::info!("Loaded file: {:?}", geojson_file_path);
    layer_ids
    */
}

fn _load_geojson_feature(
    geojson_feature: geojson::Feature,
    layers: &mut Layers,
    source_projection: &'static str,
    target_projection: &'static str,
) -> Vec<rgis_layers::LayerId> {
    if let Some(geometry) = geojson_feature.geometry {
        _load_geojson_geometry(
            geometry,
            layers,
            geojson_feature.properties,
            source_projection,
            target_projection,
        )
    } else {
        vec![]
    }
}

fn _load_geojson_geometry(
    geojson_geometry: geojson::Geometry,
    layers: &mut Layers,
    metadata: Option<rgis_layers::Metadata>,
    source_projection: &'static str,
    target_projection: &'static str,
) -> Vec<rgis_layers::LayerId> {
    let geojson_value = geojson_geometry.value;

    match geojson_value {
        g @ geojson::Value::LineString(_) => {
            let g = (g.try_into().ok() as Option<geo_types::LineString<f64>>).unwrap();
            vec![layers.add(
                geo_types::Geometry::LineString(g),
                metadata,
                source_projection,
                target_projection,
            )]
        }
        g @ geojson::Value::Polygon(_) => {
            let g = (g.try_into().ok() as Option<geo_types::Polygon<f64>>).unwrap();
            vec![layers.add(
                geo_types::Geometry::Polygon(g),
                metadata,
                source_projection,
                target_projection,
            )]
        }
        g @ geojson::Value::MultiLineString(_) => {
            let g = (g.try_into().ok() as Option<geo_types::MultiLineString<f64>>).unwrap();
            vec![layers.add(
                geo_types::Geometry::MultiLineString(g),
                metadata,
                source_projection,
                target_projection,
            )]
        }
        g @ geojson::Value::MultiPolygon(_) => {
            let g = (g.try_into().ok() as Option<geo_types::MultiPolygon<f64>>).unwrap();
            vec![layers.add(
                geo_types::Geometry::MultiPolygon(g),
                metadata,
                source_projection,
                target_projection,
            )]
        }
        geojson::Value::GeometryCollection(g) => {
            let mut layer_ids = vec![];
            for geojson_geometry in g {
                layer_ids.extend(_load_geojson_geometry(
                    geojson_geometry,
                    layers,
                    metadata.clone(),
                    source_projection,
                    target_projection,
                ));
            }
            layer_ids
        }
        _ => vec![],
    }
}
