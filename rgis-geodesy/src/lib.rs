use bevy::prelude::*;

#[derive(Resource)]
pub struct GeodesyContext(pub geodesy::Minimal);

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GeodesyContext(geodesy::Minimal::default()));
    }
}
