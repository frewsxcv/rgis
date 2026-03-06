use bevy_egui::egui;

pub struct MessageWindowContent<'a> {
    pub message: &'a str,
}

impl egui::Widget for MessageWindowContent<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label(self.message)
    }
}
