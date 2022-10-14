#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

mod systems;
mod tasks;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system_set(systems::system_set())
            .add_startup_system(set_proj_log_level);
    }
}

fn set_proj_log_level() {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        proj_sys::proj_log_level(std::ptr::null_mut(), proj_sys::PJ_LOG_LEVEL_PJ_LOG_NONE);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TransformError {
    #[cfg(target_arch = "wasm32")]
    #[error("{0}")]
    GeoProjJs(#[from] geo_proj_js::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("{0}")]
    Proj(#[from] geo::algorithm::proj::TransformError),
    #[error("{0}")]
    BoundingRect(#[from] geo_features::BoundingRectError),
}

pub fn transform<
    #[cfg(target_arch = "wasm32")]
    G: geo::MapCoordsInPlace<f64>,
    #[cfg(not(target_arch = "wasm32"))]
    G: geo::transform::Transform<f64>,
>(
    geometry: &mut G,
    source_crs: &str,
    target_crs: &str,
) -> Result<(), TransformError> {
    #[cfg(target_arch = "wasm32")]
    {
        geo_proj_js::transform(geometry, source_crs, target_crs)?;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        geometry.transform_crs_to_crs(source_crs, target_crs)?;
    }
    Ok(())
}
