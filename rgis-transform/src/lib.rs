#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

mod systems;
mod tasks;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system_set(systems::system_set())
            .add_plugin(rgis_task::TaskPlugin::<tasks::ReprojectGeometryTask>::new());
    }
}
