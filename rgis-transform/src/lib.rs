#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use geo::{Coord, MapCoords};

use geodesy::Context;
pub use geodesy::Error as TransformError;

mod jobs;
mod systems;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        systems::configure(app);
    }
}

pub struct ProjTransformer {
    source: geodesy::OpHandle,
    target: geodesy::OpHandle,
}

impl ProjTransformer {
    pub fn setup(source_crs: u16, target_crs: u16) -> Result<Self, geodesy::Error> {
        let source = crs_definitions::from_code(source_crs).unwrap();
        let target = crs_definitions::from_code(target_crs).unwrap();
        let source_geodesy_string = geodesy::parse_proj(source.proj4).unwrap();
        let target_geodesy_string = geodesy::parse_proj(target.proj4).unwrap();
        Ok(ProjTransformer {
            source: geodesy::Plain::new().op(&source_geodesy_string).unwrap(),
            target: geodesy::Plain::new().op(&target_geodesy_string).unwrap(),
        })
    }

    pub fn transform(&self, geometry: &mut geo::Geometry) -> Result<(), geodesy::Error> {
        // FIXME: use try_map_coords_in_place
        let mut transformed = geometry.try_map_coords::<geodesy::Error>(|coord| {
            let coord = geodesy::Coor2D::raw(coord.x, coord.y);
            let ctx = geodesy::Plain::new();
            ctx.apply(self.source, geodesy::Direction::Inv, &mut [coord])?;
            ctx.apply(self.target, geodesy::Direction::Fwd, &mut [coord])?;
            Ok(Coord { x: coord.0[0], y: coord.0[1] })
        })?;

        std::mem::swap(&mut transformed, geometry);

        Ok(())
    }
}
