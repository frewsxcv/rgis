#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;

mod systems;

#[derive(Clone, Resource)]
pub struct MousePos(pub geo_projected::Projected<geo::Coord>);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system_set(systems::system_set())
            .insert_resource(MousePos(geo_projected::Projected::new(geo::Coord {
                x: 0.,
                y: 0.,
            })));
    }
}
