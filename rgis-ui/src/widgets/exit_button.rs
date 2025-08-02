use bevy_egui::egui;

pub struct ExitButton<'a> {
    pub app_exit_events: &'a mut bevy::ecs::event::Events<bevy::app::AppExit>,
}

impl egui::Widget for ExitButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled_ui(cfg!(not(target_arch = "wasm32")), |ui| {
            if ui.button("Exit").clicked() {
                self.app_exit_events.send(bevy::app::AppExit::Success);
            }
        })
        .response
    }
}
