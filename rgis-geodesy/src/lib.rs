use std::sync::Arc;

use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct GeodesyContext(pub Arc<std::sync::RwLock<geodesy::ctx::Minimal>>);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GeodesyContext(Default::default()));
    }
}

pub fn epsg_code_to_geodesy_op_handle<C: geodesy::ctx::Context>(
    ctx: &mut C,
    source_crs: u16,
) -> Result<geodesy::ctx::OpHandle, Error> {
    let source =
        crs_definitions::from_code(source_crs).ok_or(Error::UnknownEpsgCode(source_crs))?;
    let source_geodesy_string = geodesy::authoring::parse_proj(source.proj4)?;
    let source_op_handle = ctx.op(&source_geodesy_string)?;
    Ok(source_op_handle)
}

#[derive(Debug)]
pub enum Error {
    Geodesy(geodesy::Error),
    UnknownEpsgCode(u16),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Geodesy(err) => write!(f, "Geodesy error: {err}"),
            Error::UnknownEpsgCode(code) => write!(f, "Unknown EPSG code: {code}"),
        }
    }
}

impl From<geodesy::Error> for Error {
    fn from(err: geodesy::Error) -> Self {
        Error::Geodesy(err)
    }
}

impl std::error::Error for Error {}
