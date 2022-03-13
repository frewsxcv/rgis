use bevy_egui::egui;

pub(crate) struct ManageLayerWindow<'a> {
    pub state: &'a mut crate::UiState,
    pub layers: &'a rgis_layers::Layers,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub color_events: &'a mut bevy::app::Events<rgis_events::UpdateLayerColor>,
}

impl<'a> ManageLayerWindow<'a> {
    pub fn render(&mut self) {
        if let (true, Some(layer_id)) = (
            self.state.is_manage_layer_window_visible,
            self.state.managing_layer,
        ) {
            let layer = match self.layers.get(layer_id) {
                Some(l) => l,
                None => {
                    bevy::log::warn!(
                        "Could not find layer with ID {:?}, closing manage layer window",
                        layer_id
                    );
                    self.state.is_manage_layer_window_visible = false;
                    return;
                }
            };
            egui::Window::new("Manage Layer")
                .open(&mut self.state.is_manage_layer_window_visible)
                .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                    egui::Grid::new("manage_layer_window_grid")
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
                                self.color_events.send(rgis_events::UpdateLayerColor(
                                    layer.id,
                                    bevy::prelude::Color::rgba_linear(
                                        old_color[0],
                                        old_color[1],
                                        old_color[2],
                                        old_color[3],
                                    ),
                                ));
                            }
                            ui.end_row();
                        });
                });
        }
    }
}
