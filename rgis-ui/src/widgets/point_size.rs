use bevy::prelude::*;
use bevy_egui::egui;
use rgis_ui_events::UpdateLayerPointSizeEvent;

pub struct PointSize<'a> {
    pub layer_id: rgis_primitives::LayerId,
    pub size: f32,
    pub size_events: &'a mut Events<UpdateLayerPointSizeEvent>,
}

impl egui::Widget for PointSize<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut new_size = self.size;
        let response = ui.add(egui::Slider::new(&mut new_size, 1.0..=50.0));
        if response.changed() {
            self.size_events
                .send(UpdateLayerPointSizeEvent(self.layer_id, new_size));
        }
        response
    }
}
