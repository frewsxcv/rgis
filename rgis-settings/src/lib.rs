use bevy::prelude::*;

mod systems;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Tool {
    Pan,
    Query,
    Measure,
}


#[derive(Resource)]
pub struct RgisSettings {
    pub current_tool: Tool,
    pub show_scale: bool,
    pub dark_mode: bool,
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let dark_mode = matches!(dark_light::detect(), Ok(dark_light::Mode::Dark));
        app.insert_resource(RgisSettings {
            current_tool: Tool::Pan,
            show_scale: true,
            dark_mode,
        });
    }
}
