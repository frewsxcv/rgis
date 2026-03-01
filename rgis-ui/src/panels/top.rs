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
                let file_response = ui.menu_button("File", |ui| {
                    ui.add(crate::widgets::exit::Exit {
                        app_exit_events: self.app_exit_events,
                    });
                });
                crate::widget_registry::register("File", file_response.response.rect);
                let view_response = ui.menu_button("View", |ui| {
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
                crate::widget_registry::register("View", view_response.response.rect);
                let help_response = ui.menu_button("Help", |ui| {
                    let debug_btn = ui.button("Debug stats");
                    crate::widget_registry::register("Debug stats", debug_btn.rect);
                    if debug_btn.clicked() {
                        self.is_debug_window_open.0 = true;
                    }
                    let source_btn = ui.button("Source code");
                    crate::widget_registry::register("Source code", source_btn.rect);
                    if source_btn.clicked() {
                        ui.ctx().open_url(egui::OpenUrl {
                            url: String::from("https://github.com/frewsxcv/rgis"),
                            new_tab: true,
                        });
                    }
                });
                crate::widget_registry::register("Help", help_response.response.rect);

                ui.separator();

                let pan_btn = ui
                    .add_enabled(
                        self.app_settings.current_tool != rgis_settings::Tool::Pan,
                        egui::Button::new("🔁 Pan Tool")
                            .selected(self.app_settings.current_tool == rgis_settings::Tool::Pan),
                    );
                crate::widget_registry::register("Pan Tool", pan_btn.rect);
                if pan_btn.clicked()
                {
                    self.app_settings.current_tool = rgis_settings::Tool::Pan;
                }

                let query_btn = ui
                    .add_enabled(
                        self.app_settings.current_tool != rgis_settings::Tool::Query,
                        egui::Button::new("ℹ Query Tool")
                            .selected(self.app_settings.current_tool == rgis_settings::Tool::Query),
                    );
                crate::widget_registry::register("Query Tool", query_btn.rect);
                if query_btn.clicked()
                {
                    self.app_settings.current_tool = rgis_settings::Tool::Query;
                }

                let measure_btn = ui
                    .add_enabled(
                        self.app_settings.current_tool != rgis_settings::Tool::Measure,
                        egui::Button::new("📏 Measure Tool")
                            .selected(self.app_settings.current_tool == rgis_settings::Tool::Measure),
                    );
                crate::widget_registry::register("Measure Tool", measure_btn.rect);
                if measure_btn.clicked()
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
