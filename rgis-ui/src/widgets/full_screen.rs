use bevy::{prelude::*, window::WindowMode};
use bevy_egui::egui;

pub struct FullScreen<'a> {
    pub window: &'a mut Window,
}

impl egui::Widget for FullScreen<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled_ui(cfg!(not(target_arch = "wasm32")), |ui| {
            if ui.button("Full screen").clicked() {
                self.window.mode = if matches!(self.window.mode, WindowMode::Fullscreen(_, _)) {
                    WindowMode::Windowed
                } else {
                    WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current)
                };
            }
        })
        .response
    }
}
