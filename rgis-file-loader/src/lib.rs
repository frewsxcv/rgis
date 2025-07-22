#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;

mod jobs;
pub mod lod_builder;
mod systems;

pub use lod_builder::{LODBuilderJob, LODBuilderJobOutcome};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        systems::configure(app);
    }
}
