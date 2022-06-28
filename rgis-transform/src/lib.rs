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
            .add_startup_system(set_proj_log_level);
    }
}

fn set_proj_log_level() {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        proj_sys::proj_log_level(std::ptr::null_mut(), proj_sys::PJ_LOG_LEVEL_PJ_LOG_NONE);
    }
}
