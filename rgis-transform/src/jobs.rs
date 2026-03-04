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

        // Sample points along each edge of a slightly inset extent to build a
        // projected bounding rect. The small inset (~4% on each side) avoids
        // projection singularities — e.g. ±90° latitude in Web Mercator maps
        // to ±infinity. Dense interior sampling also gives a more accurate
        // bounding rect for non-linear projections.
        const N: usize = 21;
        let mut sample_points = Vec::with_capacity(4 * N);
        // Inset fraction: first sample at ~1/(N+1) from each edge
        let inset = 1.0 / (N as f64 + 1.0);
        let x_lo = min.x + inset * (max.x - min.x);
        let x_hi = max.x - inset * (max.x - min.x);
        let y_lo = min.y + inset * (max.y - min.y);
        let y_hi = max.y - inset * (max.y - min.y);
        for i in 0..N {
            let t = i as f64 / (N - 1) as f64;
            let x = x_lo + t * (x_hi - x_lo);
            let y = y_lo + t * (y_hi - y_lo);
            // Bottom edge (inset)
            sample_points.push(geo::point! { x: x, y: y_lo });
            // Top edge (inset)
            sample_points.push(geo::point! { x: x, y: y_hi });
            // Left edge (inset)
            sample_points.push(geo::point! { x: x_lo, y: y });
            // Right edge (inset)
            sample_points.push(geo::point! { x: x_hi, y: y });
        }

        let mut multi_point: geo::Geometry<f64> =
            geo::MultiPoint::from(sample_points).into();

        let geodesy_ctx = self.geodesy_ctx.0.read().unwrap();
        let transformer = geo_geodesy::Transformer::from_geodesy(
            &*geodesy_ctx,
            self.source_crs.op_handle,
            self.target_crs.op_handle,
        )?;
        transformer.transform(&mut multi_point)?;

        let geo::Geometry::MultiPoint(transformed) = multi_point else {
            unreachable!()
        };

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for point in transformed.0.iter() {
            let c = point.0;
            if !c.x.is_finite() || !c.y.is_finite() {
                continue;
            }
            min_x = min_x.min(c.x);
            min_y = min_y.min(c.y);
            max_x = max_x.max(c.x);
            max_y = max_y.max(c.y);
        }

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
