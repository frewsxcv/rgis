use bevy::prelude::*;
use geo_projected::ProjectedScalar;

mod systems;

#[derive(Clone, Resource)]
pub struct MousePos(pub geo::Coord<ProjectedScalar>);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        systems::configure(app);
        app.insert_resource(MousePos(geo::Coord {
            x: num_t::Num::new(0.),
            y: num_t::Num::new(0.),
        }));
    }
}
