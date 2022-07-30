use bevy_egui::egui;

pub(crate) struct TopPanel<'a> {
    pub app_exit_events: &'a mut bevy::ecs::event::Events<bevy::app::AppExit>,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub windows: &'a mut bevy::window::Windows,
    pub app_settings: &'a mut rgis_settings::RgisSettings,
    pub top_panel_height: &'a mut crate::TopPanelHeight,
}

impl<'a> TopPanel<'a> {
    pub(crate) fn render(&mut self) {
        let inner_response =
            egui::TopBottomPanel::top("top_panel").show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("rgis");
                    ui.menu_button("File", |ui| {
                        render_exit_button(self.app_exit_events, ui);
                    });
                    ui.menu_button("View", |ui| {
                        render_full_screen_button(self.windows, ui);
                    });
                    ui.menu_button("Help", |ui| {
                        if ui.button("Source code").clicked() {
                            let _ = webbrowser::open("https://github.com/frewsxcv/rgis");
                        }
                    });

                    ui.separator();

                    if ui
                        .add_enabled(
                            self.app_settings.current_tool != rgis_settings::Tool::Pan,
                            egui::Button::new("🔁 Pan Tool"),
                        )
                        .clicked()
                    {
                        self.app_settings.current_tool = rgis_settings::Tool::Pan;
                    }

                    if ui
                        .add_enabled(
                            self.app_settings.current_tool != rgis_settings::Tool::Query,
                            egui::Button::new("ℹ Query Tool"),
                        )
                        .clicked()
                    {
                        self.app_settings.current_tool = rgis_settings::Tool::Query;
                    }
                });
            });

        self.top_panel_height.0 = inner_response.response.rect.height();
    }
}

fn render_exit_button(
    app_exit_events: &mut bevy::ecs::event::Events<bevy::app::AppExit>,
    ui: &mut egui::Ui,
) {
    ui.add_enabled_ui(cfg!(not(target_arch = "wasm32")), |ui| {
        if ui.button("Exit").clicked() {
            app_exit_events.send(bevy::app::AppExit);
        }
    });
}

fn render_full_screen_button(windows: &mut bevy::window::Windows, ui: &mut egui::Ui) {
    ui.add_enabled_ui(cfg!(not(target_arch = "wasm32")), |ui| {
        if ui.button("Full screen").clicked() {
            let window = windows.primary_mut();
            window.set_mode(if window.mode() == bevy::window::WindowMode::Fullscreen {
                bevy::window::WindowMode::Windowed
            } else {
                bevy::window::WindowMode::Fullscreen
            });
        }
    });
}
