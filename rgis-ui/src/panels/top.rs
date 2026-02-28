use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui_window as window;

pub struct Top<'a, 'w, 's> {
    pub app_exit_events: &'a mut Messages<AppExit>,
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub window: &'a mut Window,
    pub app_settings: &'a mut rgis_settings::RgisSettings,
    pub top_panel_height: &'a mut rgis_units::TopPanelHeight,
    pub is_debug_window_open: &'a mut window::IsWindowOpen<crate::windows::debug::Debug<'w, 's>>,
}

impl Top<'_, '_, '_> {
    pub fn render(&mut self) {
        let inner_response = egui::TopBottomPanel::top("top_panel").show(self.egui_ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let prev_current_tool = self.app_settings.current_tool;

                ui.label("rgis");
                ui.menu_button("File", |ui| {
                    ui.add(crate::widgets::exit::Exit {
                        app_exit_events: self.app_exit_events,
                    });
                });
                ui.menu_button("View", |ui| {
                    ui.add(crate::widgets::full_screen::FullScreen {
                        window: self.window,
                    });
                    if ui
                        .button(format!(
                            "{} scale",
                            if self.app_settings.show_scale {
                                "Hide"
                            } else {
                                "Show"
                            }
                        ))
                        .clicked()
                    {
                        self.app_settings.show_scale = !self.app_settings.show_scale;
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("Debug stats").clicked() {
                        self.is_debug_window_open.0 = true;
                    }
                    if ui.button("Source code").clicked() {
                        ui.ctx().open_url(egui::OpenUrl {
                            url: String::from("https://github.com/frewsxcv/rgis"),
                            new_tab: true,
                        });
                    }
                });

                ui.separator();

                if ui
                    .add_enabled(
                        self.app_settings.current_tool != rgis_settings::Tool::Pan,
                        egui::Button::new("🔁 Pan Tool")
                            .selected(self.app_settings.current_tool == rgis_settings::Tool::Pan),
                    )
                    .clicked()
                {
                    self.app_settings.current_tool = rgis_settings::Tool::Pan;
                }

                if ui
                    .add_enabled(
                        self.app_settings.current_tool != rgis_settings::Tool::Query,
                        egui::Button::new("ℹ Query Tool")
                            .selected(self.app_settings.current_tool == rgis_settings::Tool::Query),
                    )
                    .clicked()
                {
                    self.app_settings.current_tool = rgis_settings::Tool::Query;
                }

                if ui
                    .add_enabled(
                        self.app_settings.current_tool != rgis_settings::Tool::Measure,
                        egui::Button::new("📏 Measure Tool")
                            .selected(self.app_settings.current_tool == rgis_settings::Tool::Measure),
                    )
                    .clicked()
                {
                    self.app_settings.current_tool = rgis_settings::Tool::Measure;
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
