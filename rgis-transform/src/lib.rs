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
    ctx: geodesy::Minimal,
    source: geodesy::OpHandle,
    target: geodesy::OpHandle,
}

mod noop {
    pub const GAMUT: [geodesy::OpParameter; 0] = [];

    pub fn new(
        parameters: &geodesy::RawParameters,
        _ctx: &dyn geodesy::Context,
    ) -> Result<geodesy::Op, geodesy::Error> {
        geodesy::Op::plain(
            parameters,
            geodesy::InnerOp::default(),
            Some(geodesy::InnerOp::default()),
            &GAMUT,
            _ctx,
        )
    }
}

impl ProjTransformer {
    pub fn setup(source_crs: u16, target_crs: u16) -> Result<Self, geodesy::Error> {
        let source = crs_definitions::from_code(source_crs).unwrap();
        let target = crs_definitions::from_code(target_crs).unwrap();
        let mut ctx = geodesy::Minimal::new();
        // https://github.com/Rennzie/geodesy-wasm/blob/82f9ef050372d53144969dba60807c7dac38c910/src/geodesy/operators/mod.rs#L7-L19
        //
        // https://github.com/busstoptaktik/geodesy/issues/79
        ctx.register_op("longlat", geodesy::OpConstructor(noop::new));
        let source_geodesy_string = geodesy::parse_proj(source.proj4)?;
        let source_op_handle = ctx.op(&source_geodesy_string)?;
        let target_geodesy_string = geodesy::parse_proj(target.proj4)?;
        let target_op_handle = ctx.op(&target_geodesy_string)?;
        Ok(ProjTransformer {
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
