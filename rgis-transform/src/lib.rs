#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

mod tasks;
mod systems;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_system(systems::handle_layer_created_events)
            .add_system(systems::handle_reproject_geometry_task_completion_events)
            .add_system(systems::handle_crs_changed_events)
            .add_plugin(rgis_task::TaskPlugin::<tasks::ReprojectGeometryTask>::new());
    }
}
