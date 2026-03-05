use std::sync::Arc;

use bevy::prelude::*;

/// Shared geodesy context for coordinate reference system operations.
///
/// This resource wraps `geodesy::ctx::Minimal` in an `Arc<RwLock>` because
/// it must be cloned into background jobs (which run on `AsyncComputeTaskPool`
/// threads). Use [`Self::clone_for_async`] to obtain a clone for background
/// work, and [`Self::read`] / [`Self::write`] for synchronous system access.
#[derive(Resource)]
pub struct GeodesyContext(Arc<std::sync::RwLock<geodesy::ctx::Minimal>>);

impl GeodesyContext {
    /// Clone the context handle for use in a background job.
    ///
    /// This is the only way to obtain a second handle, making the
    /// `Arc` cloning explicit rather than implicit via `Clone`.
    pub fn clone_for_async(&self) -> Self {
        GeodesyContext(Arc::clone(&self.0))
    }

    /// Acquire a read lock on the inner geodesy context.
    pub fn read(
        &self,
    ) -> Result<std::sync::RwLockReadGuard<'_, geodesy::ctx::Minimal>, GeodesyContextLockError>
    {
        self.0.read().map_err(|_| GeodesyContextLockError)
    }

    /// Acquire a write lock on the inner geodesy context.
    pub fn write(
        &self,
    ) -> Result<std::sync::RwLockWriteGuard<'_, geodesy::ctx::Minimal>, GeodesyContextLockError>
    {
        self.0.write().map_err(|_| GeodesyContextLockError)
    }
}

/// Error returned when the `GeodesyContext` lock is poisoned.
#[derive(Debug)]
pub struct GeodesyContextLockError;

impl std::fmt::Display for GeodesyContextLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GeodesyContext lock poisoned")
    }
}

impl std::error::Error for GeodesyContextLockError {}

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

pub fn proj_string_to_geodesy_op_handle<C: geodesy::ctx::Context>(
    ctx: &mut C,
    proj_string: &str,
) -> Result<geodesy::ctx::OpHandle, Error> {
    let geodesy_string = geodesy::authoring::parse_proj(proj_string)?;
    let op_handle = ctx.op(&geodesy_string)?;
    Ok(op_handle)
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
