use crate::panels::side_panel::Events;
use bevy_egui::egui;

pub struct MoveUpMoveDownWidget<'a, 'w> {
    pub layer: &'a rgis_layers::Layer,
    pub is_move_up_enabled: bool,
    pub is_move_down_enabled: bool,
    pub events: &'a mut Events<'w>,
}

impl egui::Widget for MoveUpMoveDownWidget<'_, '_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            if ui
                .add_enabled(self.is_move_up_enabled, egui::Button::new("⬆ Move up"))
                .clicked()
            {
                self.events
                    .move_layer_event_writer
                    .write(rgis_events::MoveLayerEvent(
                        self.layer.id,
                        rgis_events::MoveDirection::Up,
                    ));
            }

            if ui
                .add_enabled(self.is_move_down_enabled, egui::Button::new("⬇ Move down"))
                .clicked()
            {
                self.events
                    .move_layer_event_writer
                    .write(rgis_events::MoveLayerEvent(
                        self.layer.id,
                        rgis_events::MoveDirection::Down,
                    ));
            }
        })
        .response
    }
}
