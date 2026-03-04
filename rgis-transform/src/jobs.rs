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

        let corners = [
            geo::coord! { x: min.x, y: min.y },
            geo::coord! { x: max.x, y: min.y },
            geo::coord! { x: max.x, y: max.y },
            geo::coord! { x: min.x, y: max.y },
        ];

        let mut multi_point: geo::Geometry<f64> =
            geo::MultiPoint::from(corners.to_vec()).into();

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
