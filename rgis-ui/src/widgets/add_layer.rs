use bevy_egui::egui;

pub struct AddLayer;

impl egui::Widget for AddLayer {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let button = ui.button("Add Layer...");
        crate::widget_registry::register("Add Layer", button.rect);
        button
    }
}
