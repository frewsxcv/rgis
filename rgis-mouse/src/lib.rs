#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

mod systems;

#[derive(Copy, Clone)]
pub struct MousePos {
    pub projected: geo::Coordinate,
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system_set(systems::system_set())
            .insert_resource(MousePos {
                projected: geo::Coordinate { x: 0., y: 0. },
            });
    }
}
