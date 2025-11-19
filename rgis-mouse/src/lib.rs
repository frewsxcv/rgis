use bevy::prelude::*;
use geo_projected::ProjectedScalar;

mod systems;

#[derive(Clone, Resource)]
pub struct MousePos(pub geo::Coord<ProjectedScalar>);

#[derive(Clone, Default, Resource)]
pub struct LastCursorScreenPosition(pub Option<rgis_units::ScreenCoord>);

#[derive(Resource, Default)]
pub struct MeasureState {
    pub start: Option<geo::Coord<ProjectedScalar>>,
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        systems::configure(app);
        app.insert_resource(MousePos(geo::Coord {
            x: num_t::Num::new(0.),
            y: num_t::Num::new(0.),
        }))
        .init_resource::<LastCursorScreenPosition>()
        .init_resource::<MeasureState>();
    }
}
