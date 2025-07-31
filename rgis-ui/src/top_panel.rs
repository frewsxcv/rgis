use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui_window as window;

pub(crate) struct TopPanel<'a, 'w, 's> {
    pub app_exit_events: &'a mut bevy::ecs::event::Events<bevy::app::AppExit>,
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub window: &'a mut Window,
    pub app_settings: &'a mut rgis_settings::RgisSettings,
    pub top_panel_height: &'a mut rgis_units::TopPanelHeight,
    pub is_debug_window_open:
        &'a mut window::IsWindowOpen<crate::debug_window::DebugWindow<'w, 's>>,
}

impl TopPanel<'_, '_, '_> {
    pub(crate) fn render(&mut self) {
        let inner_response = egui::TopBottomPanel::top("top_panel").show(self.egui_ctx, |ui| {
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
                        window: self.window,
                    });
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

impl egui::Widget for ExitButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled_ui(cfg!(not(target_arch = "wasm32")), |ui| {
            if ui.button("Exit").clicked() {
                self.app_exit_events.send(bevy::app::AppExit::Success);
            }
        })
        .response
    }
}

struct FullScreenButton<'a> {
    window: &'a mut bevy::window::Window,
}

impl egui::Widget for FullScreenButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled_ui(cfg!(not(target_arch = "wasm32")), |ui| {
            if ui.button("Full screen").clicked() {
                self.window.mode =
                    if matches!(self.window.mode, bevy::window::WindowMode::Fullscreen(_, _)) {
                        bevy::window::WindowMode::Windowed
                    } else {
                        bevy::window::WindowMode::Fullscreen(
                            MonitorSelection::Current,
                            VideoModeSelection::Current,
                        )
                    };
            }
        })
        .response
    }
}
