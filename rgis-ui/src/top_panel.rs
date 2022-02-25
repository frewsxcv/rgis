use bevy_egui::egui;

pub struct TopPanel<'a> {
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub thread_pool: &'a bevy::tasks::AsyncComputeTaskPool,
}

impl<'a> TopPanel<'a> {
    pub fn render(&mut self) {
        egui::TopBottomPanel::top("top_panel").show(self.bevy_egui_ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label("rgis");
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.thread_pool.spawn(async {
                            let task = rfd::AsyncFileDialog::new().pick_file();
                            let file_handle = task.await;
                            if let Some(n) = file_handle {
                                bevy::log::error!("fo: {:?}", n);
                            }
                        }).detach();
                    }
                });
                ui.menu_button("View", |_ui| {
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("Source code").clicked() {
                        let _ = webbrowser::open("https://github.com/frewsxcv/rgis");
                    }
                });
            });
        });
    }
}
