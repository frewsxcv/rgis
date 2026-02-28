use bevy::prelude::*;
use bevy_egui::egui;

pub struct Exit<'a> {
    pub app_exit_events: &'a mut Messages<AppExit>,
}

impl egui::Widget for Exit<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled_ui(cfg!(not(target_arch = "wasm32")), |ui| {
            if ui.button("Exit").clicked() {
                self.app_exit_events.write(AppExit::Success);
            }
        })
        .response
    }
}
