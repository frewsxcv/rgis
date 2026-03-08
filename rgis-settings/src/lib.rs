use bevy::prelude::*;

mod systems;

#[derive(States, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tool {
    #[default]
    Pan,
    Query,
    Measure,
}

#[derive(Resource)]
pub struct RgisSettings {
    pub show_scale: bool,
    pub dark_mode: bool,
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let dark_mode = matches!(dark_light::detect(), Ok(dark_light::Mode::Dark));
        app.init_state::<Tool>();
        app.insert_resource(RgisSettings {
            show_scale: true,
            dark_mode,
        });
    }
}
