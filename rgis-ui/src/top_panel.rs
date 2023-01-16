use bevy_egui::egui;

pub(crate) struct TopPanel<'a> {
    pub app_exit_events: &'a mut bevy::ecs::event::Events<bevy::app::AppExit>,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub windows: &'a mut bevy::window::Windows,
    pub app_settings: &'a mut rgis_settings::RgisSettings,
    pub top_panel_height: &'a mut crate::TopPanelHeight,
    pub debug_stats_window_state: &'a mut crate::DebugStatsWindowState,
}

impl<'a> TopPanel<'a> {
    pub(crate) fn render(&mut self) {
        let inner_response =
            egui::TopBottomPanel::top("top_panel").show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                egui::menu::bar(ui, |ui| {
                    let prev_current_tool = self.app_settings.current_tool;

                    ui.label("rgis");
                    ui.menu_button("File", |ui| {
                        ui.add(ExitButton {
                            app_exit_events: self.app_exit_events,
                        });
                    });
                    ui.menu_button("View", |ui| {
                        ui.add(FullScreenButton {
                            windows: self.windows,
                        });
                    });
                    ui.menu_button("Help", |ui| {
                        if ui.button("Debug stats").clicked() {
                            self.debug_stats_window_state.is_visible = true;
                        }
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

                    if prev_current_tool == rgis_settings::Tool::Query
                        && self.app_settings.current_tool != rgis_settings::Tool::Query
                    {
                        // send DeselectAllFeatures event
                    }
                });
            });

        self.top_panel_height.0 = inner_response.response.rect.height();
    }
}

struct ExitButton<'a> {
    app_exit_events: &'a mut bevy::ecs::event::Events<bevy::app::AppExit>,
}

impl<'a> egui::Widget for ExitButton<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled_ui(cfg!(not(target_arch = "wasm32")), |ui| {
            if ui.button("Exit").clicked() {
                self.app_exit_events.send(bevy::app::AppExit);
            }
        })
        .response
    }
}

struct FullScreenButton<'a> {
    windows: &'a mut bevy::window::Windows,
}

impl<'a> egui::Widget for FullScreenButton<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled_ui(cfg!(not(target_arch = "wasm32")), |ui| {
            if ui.button("Full screen").clicked() {
                let window = self.windows.primary_mut();
                window.set_mode(if window.mode() == bevy::window::WindowMode::Fullscreen {
                    bevy::window::WindowMode::Windowed
                } else {
                    bevy::window::WindowMode::Fullscreen
                });
            }
        })
        .response
    }
}
