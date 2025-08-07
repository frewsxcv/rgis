use bevy::prelude::*;
use bevy_egui::egui;
use rgis_ui_events::UpdateLayerColorEvent;

pub struct StrokeColor<'a> {
    pub layer_id: rgis_primitives::LayerId,
    pub color: Color,
    pub color_events: &'a mut Events<UpdateLayerColorEvent>,
}

impl egui::Widget for StrokeColor<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut old_color = self.color.to_linear().to_f32_array();
        let response = ui.color_edit_button_rgba_unmultiplied(&mut old_color);
        if response.changed() {
            self.color_events.send(UpdateLayerColorEvent::Stroke(
                self.layer_id,
                Color::linear_rgba(old_color[0], old_color[1], old_color[2], old_color[3]),
            ));
        }
        response
    }
}
