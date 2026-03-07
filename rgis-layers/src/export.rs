use geo::MapCoords;
use geozero::{ColumnValue, FeatureProcessor, GeozeroGeometry, PropertyProcessor};

pub fn export_feature_collection(
    fc: &geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
    format: rgis_primitives::ExportFormat,
) -> Result<String, geozero::error::GeozeroError> {
    let mut out: Vec<u8> = Vec::new();

    match format {
        rgis_primitives::ExportFormat::GeoJson => {
            let mut writer = geozero::geojson::GeoJsonWriter::new(&mut out);
            write_features(fc, &mut writer)?;
        }
        rgis_primitives::ExportFormat::Wkt => {
            let mut writer = geozero::wkt::WktWriter::new(&mut out);
            write_features(fc, &mut writer)?;
        }
    }

    String::from_utf8(out)
        .map_err(|_| geozero::error::GeozeroError::Geometry("Invalid UTF-8".to_string()))
}

fn write_features<W: FeatureProcessor + geozero::GeomProcessor + PropertyProcessor>(
    fc: &geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
    writer: &mut W,
) -> Result<(), geozero::error::GeozeroError> {
    writer.dataset_begin(None)?;

    for (idx, feature) in fc.features.iter().enumerate() {
        writer.feature_begin(idx as u64)?;

        if let Some(ref geom) = feature.geometry {
            writer.geometry_begin()?;
            let geom_f64: geo::Geometry<f64> =
                geom.map_coords(|coord| geo::Coord { x: coord.x.0, y: coord.y.0 });
            geom_f64.process_geom(writer)?;
            writer.geometry_end()?;
        }

        writer.properties_begin()?;
        if let Some(ref record_batch) = fc.properties {
            let props = geo_features::properties_for_row(record_batch, idx);
            for (i, (key, value)) in props.iter().enumerate() {
                writer.property(i, key, &ColumnValue::String(value))?;
            }
        }
        writer.properties_end()?;

        writer.feature_end(idx as u64)?;
    }

    writer.dataset_end()?;
    Ok(())
}
