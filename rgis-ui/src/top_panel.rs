use bevy_egui::egui;

pub struct TopPanel<'a> {
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
}

impl<'a> TopPanel<'a> {
    pub fn render(&mut self) {
        egui::TopBottomPanel::top("top_panel").show(self.bevy_egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("rgis");
                ui.menu_button("File", |_ui| {});
                ui.menu_button("View", |_ui| {});
                ui.menu_button("Help", |ui| {
                    if ui.button("Source code").clicked() {
                        let _ = webbrowser::open("https://github.com/frewsxcv/rgis");
                    }
                });
            });
        });
    }
}
