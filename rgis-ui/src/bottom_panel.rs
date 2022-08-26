use bevy_egui::egui;

pub(crate) struct BottomPanel<'a, 'w, 's> {
    pub egui_ctx: &'a egui::Context,
    pub mouse_pos: &'a rgis_mouse::MousePos,
    pub rgis_settings: &'a rgis_settings::RgisSettings,
    pub open_change_crs_window_event_writer:
        &'a mut bevy::ecs::event::EventWriter<'w, 's, rgis_events::OpenChangeCrsWindow>,
    pub bottom_panel_height: &'a mut crate::BottomPanelHeight,
}

impl<'a, 'w, 's> BottomPanel<'a, 'w, 's> {
    pub(crate) fn render(&mut self) {
        let inner_response = egui::TopBottomPanel::bottom("bottom").show(self.egui_ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.render_crs(ui);
                    self.render_mouse_position(ui);
                });
            });
        });
        self.bottom_panel_height.0 = inner_response.response.rect.height();
    }

    fn render_crs(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.horizontal(|ui| {
                // TODO: The ordering is backwards here (the edit button should be specified after)
                //       Is this from the right_to_left call above?
                let button_response =
                    ui.add_enabled(cfg!(not(target_arch = "wasm32")), egui::Button::new("✏"));

                if button_response.clicked() {
                    self.open_change_crs_window_event_writer.send_default();
                }

                ui.label(format!("🌍 CRS: {}", self.rgis_settings.target_crs));
            });
        });
    }

    fn render_mouse_position(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.label(format!(
                "🖱 XY: {}, {}",
                self.mouse_pos.projected.x, self.mouse_pos.projected.y
            ));
        });
    }
}
