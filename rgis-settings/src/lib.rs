use bevy::prelude::*;

mod systems;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Tool {
    Pan,
    Query,
}

#[derive(Resource)]
pub struct RgisSettings {
    pub current_tool: Tool,
    pub show_scale: bool,
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(RgisSettings {
            current_tool: Tool::Pan,
            show_scale: true,
        });
    }
}
