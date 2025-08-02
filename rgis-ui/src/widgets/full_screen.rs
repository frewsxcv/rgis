use bevy::window::{MonitorSelection, VideoModeSelection};
use bevy_egui::egui;

pub struct FullScreen<'a> {
    pub window: &'a mut bevy::window::Window,
}

impl egui::Widget for FullScreen<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled_ui(cfg!(not(target_arch = "wasm32")), |ui| {
            if ui.button("Full screen").clicked() {
                self.window.mode =
                    if matches!(self.window.mode, bevy::window::WindowMode::Fullscreen(_, _)) {
                        bevy::window::WindowMode::Windowed
                    } else {
                        bevy::window::WindowMode::Fullscreen(
                            MonitorSelection::Current,
                            VideoModeSelection::Current,
                        )
                    };
            }
        })
        .response
    }
}
