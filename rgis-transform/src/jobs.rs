use std::sync::Arc;
use geo_projected::CastTo;
use geodesy::prelude::Context;

pub struct ReprojectRasterExtentJob {
    pub extent: geo::Rect<f64>,
    pub layer_id: rgis_primitives::LayerId,
    pub source_crs: rgis_primitives::Crs,
    pub target_crs: rgis_primitives::Crs,
    pub geodesy_ctx: rgis_crs::GeodesyContext,
}

pub struct ReprojectRasterExtentJobOutcome {
    pub projected_grid: rgis_layers::ProjectedRasterGrid,
    pub layer_id: rgis_primitives::LayerId,
    pub target_crs: rgis_primitives::Crs,
}

const GRID: u32 = 32;

impl bevy_jobs::Job for ReprojectRasterExtentJob {
    type Outcome = Result<ReprojectRasterExtentJobOutcome, geo_geodesy::Error>;

    fn name(&self) -> String {
        "Projecting raster extent".to_string()
    }

    async fn perform(self, _progress_sender: bevy_jobs::Context) -> Self::Outcome {
        let mut min = self.extent.min();
        let mut max = self.extent.max();

        let geodesy_ctx = self.geodesy_ctx.read().unwrap();

        // Determine if the source CRS is geographic (degrees) or projected (meters).
        // Geographic CRS inputs need degree-to-radian conversion; projected do not.
        let source_is_geographic = self.source_crs.is_geographic();

        // If the target CRS is Mercator, clamp geographic source latitudes to
        // ±85.06° — the defined area of use for EPSG:3857. Mercator Y → ∞ at
        // the poles, so without clamping near-polar vertices stretch the extent.
        if source_is_geographic {
            let target_is_mercator = self.target_crs.is_mercator();
            if target_is_mercator {
                const MERCATOR_LAT_LIMIT: f64 = 85.06;
                min.y = min.y.max(-MERCATOR_LAT_LIMIT);
                max.y = max.y.min(MERCATOR_LAT_LIMIT);
            }
        }

        let num_verts = ((GRID + 1) * (GRID + 1)) as usize;
        let mut positions = Vec::with_capacity(num_verts);
        let mut valid = Vec::with_capacity(num_verts);

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for row in 0..=GRID {
            let t_y = row as f64 / GRID as f64;
            let src_y = min.y + t_y * (max.y - min.y);
            for col in 0..=GRID {
                let t_x = col as f64 / GRID as f64;
                let src_x = min.x + t_x * (max.x - min.x);

                // Use Coor2D::gis() for geographic CRS (converts degrees to
                // radians) and Coor2D::raw() for projected CRS (meters/feet
                // passed directly). geo-geodesy's Transformer always uses
                // gis(), which corrupts projected coordinates.
                let mut coord = if source_is_geographic {
                    [geodesy::coord::Coor2D::gis(src_x, src_y)]
                } else {
                    [geodesy::coord::Coor2D::raw(src_x, src_y)]
                };
                let inv_ok = geodesy_ctx
                    .apply(self.source_crs.op_handle, geodesy::Direction::Inv, &mut coord)
                    .is_ok();
                let fwd_ok = inv_ok
                    && geodesy_ctx
                        .apply(self.target_crs.op_handle, geodesy::Direction::Fwd, &mut coord)
                        .is_ok();

                if fwd_ok {
                    let (x, y) = if self.target_crs.is_geographic() {
                        // Geographic CRS: geodesy outputs radians, convert to degrees
                        (coord[0].0[0].to_degrees(), coord[0].0[1].to_degrees())
                    } else {
                        // Projected CRS: geodesy outputs linear units (e.g. metres)
                        (coord[0].0[0], coord[0].0[1])
                    };
                    if x.is_finite() && y.is_finite() {
                        positions.push([x as f32, y as f32]);
                        valid.push(true);
                        if x < min_x { min_x = x; }
                        if y < min_y { min_y = y; }
                        if x > max_x { max_x = x; }
                        if y > max_y { max_y = y; }
                    } else {
                        positions.push([0.0, 0.0]);
                        valid.push(false);
                    }
                } else {
                    positions.push([0.0, 0.0]);
                    valid.push(false);
                }
            }
        }

        // Filter outlier positions using IQR-based detection.
        // This handles near-polar Mercator vertices and other projection
        // singularities that produce extreme but finite values.
        let mut valid_xs: Vec<f64> = Vec::new();
        let mut valid_ys: Vec<f64> = Vec::new();
        for (i, &is_valid) in valid.iter().enumerate() {
            if is_valid {
                valid_xs.push(positions[i][0] as f64);
                valid_ys.push(positions[i][1] as f64);
            }
        }

        if valid_xs.len() >= 4 {
            valid_xs.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            valid_ys.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

            let n = valid_xs.len();
            let q1_x = valid_xs[n / 4];
            let q3_x = valid_xs[3 * n / 4];
            let iqr_x = q3_x - q1_x;
            let q1_y = valid_ys[n / 4];
            let q3_y = valid_ys[3 * n / 4];
            let iqr_y = q3_y - q1_y;

            let lo_x = q1_x - 3.0 * iqr_x;
            let hi_x = q3_x + 3.0 * iqr_x;
            let lo_y = q1_y - 3.0 * iqr_y;
            let hi_y = q3_y + 3.0 * iqr_y;

            // Re-filter and recompute extent
            min_x = f64::INFINITY;
            min_y = f64::INFINITY;
            max_x = f64::NEG_INFINITY;
            max_y = f64::NEG_INFINITY;

            for (i, is_valid) in valid.iter_mut().enumerate() {
                if *is_valid {
                    let x = positions[i][0] as f64;
                    let y = positions[i][1] as f64;
                    if x < lo_x || x > hi_x || y < lo_y || y > hi_y {
                        *is_valid = false;
                    } else {
                        if x < min_x { min_x = x; }
                        if y < min_y { min_y = y; }
                        if x > max_x { max_x = x; }
                        if y > max_y { max_y = y; }
                    }
                }
            }
        }

        let extent = geo::Rect::new(
            geo::coord! { x: min_x, y: min_y },
            geo::coord! { x: max_x, y: max_y },
        );

        Ok(ReprojectRasterExtentJobOutcome {
            projected_grid: rgis_layers::ProjectedRasterGrid {
                cols: GRID,
                rows: GRID,
                positions,
                valid,
                extent,
            },
            layer_id: self.layer_id,
            target_crs: self.target_crs,
        })
    }
}

pub struct ReprojectGeometryJob {
    pub feature_collection: Arc<geo_features::FeatureCollection<geo_projected::UnprojectedScalar>>,
    pub layer_id: rgis_primitives::LayerId,
    pub source_crs: rgis_primitives::Crs,
    pub target_crs: rgis_primitives::Crs,
    pub geodesy_ctx: rgis_crs::GeodesyContext,
}

pub struct ReprojectGeometryJobOutcome {
    pub feature_collection: geo_features::FeatureCollection<geo_projected::ProjectedScalar>,
    pub layer_id: rgis_primitives::LayerId,
    pub target_crs: rgis_primitives::Crs,
}

impl bevy_jobs::Job for ReprojectGeometryJob {
    type Outcome = Result<ReprojectGeometryJobOutcome, geo_geodesy::Error>;

    fn name(&self) -> String {
        "Projecting layer".to_string()
    }

    async fn perform(self, progress_sender: bevy_jobs::Context) -> Self::Outcome {
        let feature_collection = Arc::unwrap_or_clone(self.feature_collection);
        let total = feature_collection.features.len();

        let mut feature_collection = feature_collection.cast::<geo_projected::Projected>();

        for (i, feature) in feature_collection.features.iter_mut().enumerate() {
            let _ = progress_sender.send_progress((100 * i / total) as u8).await;

            let geodesy_ctx = self.geodesy_ctx.read().unwrap();

            let transformer = geo_geodesy::Transformer::from_geodesy(
                &*geodesy_ctx,
                self.source_crs.op_handle,
                self.target_crs.op_handle,
                self.target_crs.is_geographic(),
            )?;

            if let Some(ref mut geometry) = &mut feature.geometry {
                transformer.transform(geometry)?;
            }

            feature.recalculate_bounding_rect();
        }

        feature_collection.recalculate_bounding_rect();

        Ok(ReprojectGeometryJobOutcome {
            feature_collection,
            layer_id: self.layer_id,
            target_crs: self.target_crs,
        })
    }
}
