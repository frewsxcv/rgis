#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::Update;
use std::error;

mod jobs;
mod systems;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        systems::configure(app);
        app.add_systems(Update, set_proj_log_level);
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
    #[error("{0}")]
    SetupError(Box<dyn error::Error + Send + Sync>),
    #[cfg(target_arch = "wasm32")]
    #[error("{0}")]
    GeoProjJs(#[from] geo_proj_js::Error),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("{0}")]
    Proj(#[from] geo::algorithm::proj::TransformError),
}

pub trait Transformer {
    fn setup(
        source_crs: &str,
        target_crs: &str,
    ) -> Result<Self, Box<dyn error::Error + Send + Sync>>
    where
        Self: Sized;
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
