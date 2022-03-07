use bevy_egui::egui;

pub(crate) struct BottomPanel<'a> {
    pub egui_ctx: &'a egui::CtxRef,
    pub state: &'a mut crate::UiState,
    pub mouse_pos: &'a rgis_mouse::MousePos,
}

impl<'a> BottomPanel<'a> {
    pub fn render(&mut self) {
        egui::TopBottomPanel::bottom("bottom").show(self.egui_ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                    self.render_crs(ui);
                    self.render_mouse_position(ui);
                });
            });
        });
    }

    fn render_crs(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.label(format!("🌍 CRS: {}", self.state.target_srs));
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
