use geo_projected::CastTo;

pub struct ReprojectRasterExtentJob {
    pub extent: geo::Rect<f64>,
    pub layer_id: rgis_primitives::LayerId,
    pub source_crs: rgis_primitives::Crs,
    pub target_crs: rgis_primitives::Crs,
    pub geodesy_ctx: rgis_geodesy::GeodesyContext,
}

pub struct ReprojectRasterExtentJobOutcome {
    pub projected_extent: geo::Rect<f64>,
    pub layer_id: rgis_primitives::LayerId,
    pub target_crs: rgis_primitives::Crs,
}

impl bevy_jobs::Job for ReprojectRasterExtentJob {
    type Outcome = Result<ReprojectRasterExtentJobOutcome, geo_geodesy::Error>;

    fn name(&self) -> String {
        "Projecting raster extent".to_string()
    }

    async fn perform(self, _progress_sender: bevy_jobs::Context) -> Self::Outcome {
        let min = self.extent.min();
        let max = self.extent.max();

        // Sample points densely along each edge of the extent. Each point is
        // transformed individually so that projection singularities (e.g. ±90°
        // latitude in Web Mercator) cause that single point to be skipped
        // rather than failing the entire batch.
        const N: usize = 21;
        let mut sample_points = Vec::with_capacity(4 * N);
        for i in 0..N {
            let t = i as f64 / (N - 1) as f64;
            let x = min.x + t * (max.x - min.x);
            let y = min.y + t * (max.y - min.y);
            sample_points.push(geo::coord! { x: x, y: min.y }); // bottom
            sample_points.push(geo::coord! { x: x, y: max.y }); // top
            sample_points.push(geo::coord! { x: min.x, y: y }); // left
            sample_points.push(geo::coord! { x: max.x, y: y }); // right
        }

        let geodesy_ctx = self.geodesy_ctx.0.read().unwrap();
        let transformer = geo_geodesy::Transformer::from_geodesy(
            &*geodesy_ctx,
            self.source_crs.op_handle,
            self.target_crs.op_handle,
        )?;

        let mut xs = Vec::with_capacity(sample_points.len());
        let mut ys = Vec::with_capacity(sample_points.len());

        for coord in &sample_points {
            let mut geom: geo::Geometry<f64> = geo::Point::from(*coord).into();
            if transformer.transform(&mut geom).is_err() {
                continue;
            }
            let geo::Geometry::Point(p) = geom else {
                continue;
            };
            let c = p.0;
            if c.x.is_finite() && c.y.is_finite() {
                xs.push(c.x);
                ys.push(c.y);
            }
        }

        // Use densest-cluster analysis to exclude outliers caused by
        // projection singularities (e.g. near-polar Mercator Y values).
        let (min_x, max_x) = robust_range(&mut xs);
        let (min_y, max_y) = robust_range(&mut ys);
        let projected_extent = geo::Rect::new(
            geo::coord! { x: min_x, y: min_y },
            geo::coord! { x: max_x, y: max_y },
        );

        Ok(ReprojectRasterExtentJobOutcome {
            projected_extent,
            layer_id: self.layer_id,
            target_crs: self.target_crs,
        })
    }
}

pub struct ReprojectGeometryJob {
    pub feature_collection: geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
    pub layer_id: rgis_primitives::LayerId,
    pub source_crs: rgis_primitives::Crs,
    pub target_crs: rgis_primitives::Crs,
    pub geodesy_ctx: rgis_geodesy::GeodesyContext,
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
        let total = self.feature_collection.features.len();

        let mut feature_collection = self.feature_collection.cast::<geo_projected::Projected>();

        for (i, feature) in feature_collection.features.iter_mut().enumerate() {
            let _ = progress_sender.send_progress((100 * i / total) as u8).await;

            let geodesy_ctx = self.geodesy_ctx.0.read().unwrap();

            let transformer = geo_geodesy::Transformer::from_geodesy(
                &*geodesy_ctx,
                self.source_crs.op_handle,
                self.target_crs.op_handle,
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

/// Return a robust (min, max) range by finding the densest cluster of values
/// and expanding it, filtering out extreme outliers caused by projection
/// singularities (e.g. near-polar Mercator Y values).
///
/// When extreme outliers make up a large fraction of the data (even >50%),
/// this finds the shortest interval containing 40% of values, then expands
/// it to capture the full inlier range.
fn robust_range(values: &mut [f64]) -> (f64, f64) {
    values.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let n = values.len();
    if n < 4 {
        return (values[0], values[n - 1]);
    }

    // Find the shortest interval containing 40% of values (the densest cluster).
    let window = (n * 2 / 5).max(2);
    let mut best_start = 0;
    let mut best_width = f64::INFINITY;
    for start in 0..=(n - window) {
        let width = values[start + window - 1] - values[start];
        if width < best_width {
            best_width = width;
            best_start = start;
        }
    }

    let full_range = values[n - 1] - values[0];

    // If the densest cluster spans a reasonable fraction of the full range,
    // there are no extreme outliers — use the full range.
    if full_range <= 0.0 || best_width >= full_range * 0.1 {
        return (values[0], values[n - 1]);
    }

    // Extreme outliers detected. Expand the dense cluster with 100% padding
    // to capture the full inlier range without including outliers.
    let center = (values[best_start] + values[best_start + window - 1]) / 2.0;
    let expanded_half = best_width; // 100% padding = full width on each side
    let lo = center - expanded_half;
    let hi = center + expanded_half;

    let min = values.iter().copied().find(|&v| v >= lo).unwrap_or(values[0]);
    let max = values
        .iter()
        .copied()
        .rev()
        .find(|&v| v <= hi)
        .unwrap_or(values[n - 1]);
    (min, max)
}
