#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use geo::{Coord, MapCoords};
use std::error;

mod jobs;
mod systems;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        systems::configure(app);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TransformError {
    #[error("{0}")]
    SetupError(Box<dyn error::Error + Send + Sync>),
    #[error("{0}")]
    Proj4rs(#[from] proj4rs::errors::Error),
}

pub struct ProjTransformer {
    source: proj4rs::Proj,
    target: proj4rs::Proj,
}

impl ProjTransformer {
    // TODO: Remove the Box error return value
    pub fn setup(
        source_crs: u16,
        target_crs: u16,
    ) -> Result<Self, Box<dyn error::Error + Send + Sync>> {
        Ok(ProjTransformer {
            source: proj4rs::Proj::from_epsg_code(source_crs)?,
            target: proj4rs::Proj::from_epsg_code(target_crs)?,
        })
    }

    pub fn transform(&self, geometry: &mut geo::Geometry) -> Result<(), crate::TransformError> {
        // TODO: Replace with try_map_coords_inplace
        let mut transformed = geometry.try_map_coords::<crate::TransformError>(|mut coord| {
            if self.source.is_latlong() {
                coord.x = coord.x.to_radians();
                coord.y = coord.y.to_radians();
            }
            let (x, y) =
                proj4rs::adaptors::transform_xy(&self.source, &self.target, coord.x, coord.y)?;
            Ok(Coord { x, y })
        })?;

        std::mem::swap(&mut transformed, geometry);

        Ok(())
    }
}
