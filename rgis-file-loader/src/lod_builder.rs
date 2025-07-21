use bevy::log::error;
use bevy_jobs::{AsyncReturn, Context, Job};
use geo::{algorithm::simplify::Simplify, CoordsIter, MapCoords};
use geo_features::FeatureCollection;
use geo_projected::ProjectedScalar;
use rgis_layers::ProjectedFeatureCollectionWithLOD;
use std::collections::BTreeMap;

pub struct LODBuilderJob {
    pub projected: FeatureCollection<ProjectedScalar>,
    pub layer_id: rgis_layer_id::LayerId,
}

impl Job for LODBuilderJob {
    type Outcome = Result<LODBuilderJobOutcome, ()>;
    const JOB_TYPE: bevy_jobs::JobType = bevy_jobs::JobType::Compute;

    fn name(&self) -> String {
        "Building LODs".to_string()
    }

    fn perform(self, _: Context) -> AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            Ok(LODBuilderJobOutcome {
                lods: build_lods(self.projected),
                layer_id: self.layer_id,
            })
        })
    }
}

pub struct LODBuilderJobOutcome {
    pub lods: ProjectedFeatureCollectionWithLOD,
    pub layer_id: rgis_layer_id::LayerId,
}

fn calculate_epsilon(lod_level: u8) -> f64 {
    // TODO: make this configurable
    10.0_f64.powf(f64::from(lod_level) - 5.0)
}

pub fn build_lods(
    projected: FeatureCollection<ProjectedScalar>,
) -> ProjectedFeatureCollectionWithLOD {
    let mut lods = BTreeMap::new();

    lods.insert(0, projected.clone());

    let original_coords_count = projected.coords_count();

    for lod_level in 1..=10 {
        let epsilon = calculate_epsilon(lod_level);
        let simplified_features: Vec<_> = projected
            .features
            .iter()
            .map(|feature| {
                let mut new_feature = feature.clone();
                if let Some(geometry) = &feature.geometry {
                    // Convert to f64 geometry for simplification
                    let geom_f64: geo::Geometry<f64> =
                        geometry.map_coords(|c| (c.x.0, c.y.0).into());
                    // Simplify
                    let simplified_f64 = match geom_f64 {
                        geo::Geometry::Polygon(p) => geo::Geometry::Polygon(p.simplify(&epsilon)),
                        geo::Geometry::MultiPolygon(mp) => {
                            geo::Geometry::MultiPolygon(mp.simplify(&epsilon))
                        }
                        geo::Geometry::LineString(ls) => {
                            geo::Geometry::LineString(ls.simplify(&epsilon))
                        }
                        geo::Geometry::MultiLineString(mls) => {
                            geo::Geometry::MultiLineString(mls.simplify(&epsilon))
                        }
                        _ => geom_f64.clone(),
                    };
                    // Convert back to ProjectedScalar geometry
                    let simplified_projected: geo::Geometry<ProjectedScalar> = simplified_f64
                        .map_coords(|c| {
                            geo::Coord::from((
                                geo_projected::ProjectedScalar::from(c.x),
                                geo_projected::ProjectedScalar::from(c.y),
                            ))
                        });
                    new_feature.geometry = Some(simplified_projected);
                }
                new_feature
            })
            .collect();

        let feature_collection = FeatureCollection {
            features: simplified_features,
            ..projected.clone()
        };

        let simplified_coords_count = feature_collection.coords_count();
        let omitted_count = original_coords_count - simplified_coords_count;
        let percentage_omitted = (omitted_count as f64 / original_coords_count as f64) * 100.0;

        error!(
            "LOD {}: omitted {} points ({:.2}%)",
            lod_level, omitted_count, percentage_omitted
        );

        lods.insert(lod_level, feature_collection);
    }

    ProjectedFeatureCollectionWithLOD { projected: lods }
}
