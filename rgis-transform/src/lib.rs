#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use geo::{Coord, MapCoords};

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
    Proj4rs(#[from] proj4rs::errors::Error),
}

pub struct ProjTransformer {
    source: proj4rs::Proj,
    target: proj4rs::Proj,
}

impl ProjTransformer {
    pub fn setup(source_crs: u16, target_crs: u16) -> proj4rs::errors::Result<Self> {
        Ok(ProjTransformer {
            source: proj4rs::Proj::from_epsg_code(source_crs)?,
            target: proj4rs::Proj::from_epsg_code(target_crs)?,
        })
    }

    pub fn transform(&self, geometry: &mut geo::Geometry) -> proj4rs::errors::Result<()> {
        // FIXME: use try_map_coords_in_place
        let mut transformed = geometry.try_map_coords::<proj4rs::errors::Error>(|mut coord| {
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
