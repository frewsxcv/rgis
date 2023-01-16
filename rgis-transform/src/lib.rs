#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

mod jobs;
mod systems;

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

pub trait Transformer {
    fn setup(source_crs: &str, target_crs: &str) -> Self;
    fn transform(&self, geometry: &mut geo::Geometry) -> Result<(), TransformError>;
}

#[cfg(target_arch = "wasm32")]
mod proj_js_transformer;
#[cfg(target_arch = "wasm32")]
pub type DefaultTransformer = proj_js_transformer::ProjJsTransformer;

#[cfg(not(target_arch = "wasm32"))]
mod proj_transformer;
#[cfg(not(target_arch = "wasm32"))]
pub type DefaultTransformer = proj_transformer::ProjTransformer;
