#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use geo::{Coord, MapCoords};

pub use geodesy::{Context, Minimal, OpHandle};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // https://github.com/georust/geojson/issues/197
    #[error("{0}")]
    Geodesy(#[from] geodesy::Error),
    #[error("Unknown EPSG code: {0}")]
    UnknownEpsgCode(u16),
}

pub struct Transformer {
    ctx: geodesy::Minimal,
    source: geodesy::OpHandle,
    target: geodesy::OpHandle,
}

impl Transformer {
    pub fn setup(source_crs: u16, target_crs: u16) -> Result<Self, Error> {
        let source =
            crs_definitions::from_code(source_crs).ok_or(Error::UnknownEpsgCode(source_crs))?;
        let target =
            crs_definitions::from_code(target_crs).ok_or(Error::UnknownEpsgCode(target_crs))?;
        let mut ctx = geodesy_ctx();
        let source_geodesy_string = geodesy::parse_proj(source.proj4)?;
        let source_op_handle = ctx.op(&source_geodesy_string)?;
        let target_geodesy_string = geodesy::parse_proj(target.proj4)?;
        let target_op_handle = ctx.op(&target_geodesy_string)?;
        Ok(Transformer {
            ctx,
            source: source_op_handle,
            target: target_op_handle,
        })
    }

    pub fn transform(&self, geometry: &mut geo::Geometry) -> Result<(), geodesy::Error> {
        // FIXME: use try_map_coords_in_place
        let mut transformed = geometry.try_map_coords::<geodesy::Error>(|coord| {
            let mut coord = [geodesy::Coor2D::gis(coord.x, coord.y)];
            self.ctx
                .apply(self.source, geodesy::Direction::Inv, &mut coord)?;
            self.ctx
                .apply(self.target, geodesy::Direction::Fwd, &mut coord)?;
            Ok(Coord {
                x: coord[0].0[0],
                y: coord[0].0[1],
            })
        })?;

        std::mem::swap(&mut transformed, geometry);

        Ok(())
    }
}

pub fn lookup_epsg_code(epsg_code: u16) -> Result<(geodesy::Minimal, geodesy::OpHandle), Error> {
    let mut ctx = geodesy_ctx();
    let def = crs_definitions::from_code(epsg_code).ok_or(Error::UnknownEpsgCode(epsg_code))?;
    let source_geodesy_string = geodesy::parse_proj(def.proj4)?;
    let op_handle = ctx.op(&source_geodesy_string)?;
    Ok((ctx, op_handle))
}

fn geodesy_ctx() -> geodesy::Minimal {
    geodesy::Minimal::new()
}
