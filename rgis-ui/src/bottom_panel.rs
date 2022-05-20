use bevy_egui::egui;

pub(crate) struct BottomPanel<'a> {
    pub egui_ctx: &'a egui::Context,
    pub mouse_pos: &'a rgis_mouse::MousePos,
    pub rgis_settings: &'a rgis_settings::RgisSettings,
}

impl<'a> BottomPanel<'a> {
    pub(crate) fn render(&mut self) {
        let inner_response = egui::TopBottomPanel::bottom("bottom").show(self.egui_ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    self.render_crs(ui);
                    self.render_mouse_position(ui);
                });
            });
        });
        if inner_response.response.changed() {
            println!("resized");
            // set height of resource?
        }
    }

    fn render_crs(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.label(format!("🌍 CRS: {}", self.rgis_settings.target_crs));
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
