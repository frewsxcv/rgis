use bevy_egui::egui;

pub(crate) struct ManageLayerWindow<'a> {
    pub ui_state: &'a mut crate::UiState,
    pub rgis_layers_resource: &'a rgis_layers::ArcLayers,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub toggle_material_events: &'a mut bevy::app::Events<rgis_events::ToggleMaterialEvent>,
}

impl<'a> ManageLayerWindow<'a> {
    pub fn render(&mut self) {
        match (
            self.ui_state.layer_window_visible,
            self.ui_state.managing_layer,
        ) {
            (true, Some(layer_id)) => {
                let layers = self.rgis_layers_resource.read().unwrap(); // TODO: remove unwrap
                let layer = layers.get(layer_id).unwrap(); // TOOD: remove unwrap
                egui::Window::new("Manage Layer")
                    .open(&mut self.ui_state.layer_window_visible)
                    .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                        egui::Grid::new("FIXME")
                            .num_columns(2)
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Name");
                                ui.label(layer.name.clone());
                                ui.end_row();
                                ui.label("Color");
                                let mut old_color = layer.color.as_linear_rgba_f32();
                                if ui
                                    .color_edit_button_rgba_unmultiplied(&mut old_color)
                                    .changed()
                                {
                                    self.toggle_material_events.send(
                                        rgis_events::ToggleMaterialEvent::ChangeColor(
                                            layer.id,
                                            bevy::prelude::Color::rgba_linear(
                                                old_color[0],
                                                old_color[1],
                                                old_color[2],
                                                old_color[3],
                                            ),
                                        ),
                                    );
                                }
                                ui.end_row();
                            });
                    });
            }
            _ => (),
        }
    }
}
