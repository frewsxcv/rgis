use geo_projected::CastTo;

pub struct ReprojectRasterExtentJob {
    pub extent: geo::Rect<f64>,
    pub layer_id: rgis_primitives::LayerId,
    pub source_crs: rgis_primitives::Crs,
    pub target_crs: rgis_primitives::Crs,
    pub geodesy_ctx: rgis_geodesy::GeodesyContext,
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
        let min = self.extent.min();
        let max = self.extent.max();
        let num_verts = ((GRID + 1) * (GRID + 1)) as usize;
        let mut positions = Vec::with_capacity(num_verts);
        let mut valid = Vec::with_capacity(num_verts);

        let geodesy_ctx = self.geodesy_ctx.0.read().unwrap();
        let transformer = geo_geodesy::Transformer::from_geodesy(
            &*geodesy_ctx,
            self.source_crs.op_handle,
            self.target_crs.op_handle,
        )?;

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

                let mut geom: geo::Geometry<f64> =
                    geo::Point::new(src_x, src_y).into();
                let ok = transformer.transform(&mut geom).is_ok();
                if ok {
                    let geo::Geometry::Point(p) = geom else {
                        positions.push([0.0, 0.0]);
                        valid.push(false);
                        continue;
                    };
                    let c = p.0;
                    if c.x.is_finite() && c.y.is_finite() {
                        positions.push([c.x as f32, c.y as f32]);
                        valid.push(true);
                        if c.x < min_x { min_x = c.x; }
                        if c.y < min_y { min_y = c.y; }
                        if c.x > max_x { max_x = c.x; }
                        if c.y > max_y { max_y = c.y; }
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
