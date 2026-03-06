use bevy_egui::egui;

pub enum TopPanelAction {
    ShowAddLayerWindow,
    Exit,
    ToggleFullScreen,
    ToggleShowScale,
    ToggleDarkMode,
    OpenDebugWindow,
    OpenSourceCode,
    SetTool(rgis_settings::Tool),
}

pub struct Top<'a> {
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub current_tool: rgis_settings::Tool,
    pub show_scale: bool,
    pub top_panel_height: &'a mut rgis_units::TopPanelHeight,
}

impl Top<'_> {
    pub fn render(&mut self) -> Vec<TopPanelAction> {
        let mut actions = vec![];
        let current_tool = self.current_tool;
        let inner_response = egui::TopBottomPanel::top("top_panel").show(self.egui_ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let mut new_tool = current_tool;

                ui.label("rgis");
                let file_response = ui.menu_button("File", |ui| {
                    if ui.button("Add Layer...").clicked() {
                        actions.push(TopPanelAction::ShowAddLayerWindow);
                        ui.close();
                    }
                    ui.separator();
                    if ui
                        .add_enabled(cfg!(not(target_arch = "wasm32")), egui::Button::new("Exit"))
                        .clicked()
                    {
                        actions.push(TopPanelAction::Exit);
                    }
                });
                crate::widget_registry::register("File", file_response.response.rect);
                let view_response = ui.menu_button("View", |ui| {
                    if ui
                        .add_enabled(
                            cfg!(not(target_arch = "wasm32")),
                            egui::Button::new("Full screen"),
                        )
                        .clicked()
                    {
                        actions.push(TopPanelAction::ToggleFullScreen);
                    }
                    if ui
                        .button(format!(
                            "{} scale",
                            if self.show_scale { "Hide" } else { "Show" }
                        ))
                        .clicked()
                    {
                        actions.push(TopPanelAction::ToggleShowScale);
                    }
                    if ui.button("Toggle dark mode").clicked() {
                        actions.push(TopPanelAction::ToggleDarkMode);
                        ui.close();
                    }
                });
                crate::widget_registry::register("View", view_response.response.rect);
                let help_response = ui.menu_button("Help", |ui| {
                    let debug_btn = ui.button("Debug stats");
                    crate::widget_registry::register("Debug stats", debug_btn.rect);
                    if debug_btn.clicked() {
                        actions.push(TopPanelAction::OpenDebugWindow);
                    }
                    let source_btn = ui.button("Source code");
                    crate::widget_registry::register("Source code", source_btn.rect);
                    if source_btn.clicked() {
                        actions.push(TopPanelAction::OpenSourceCode);
                    }
                });
                crate::widget_registry::register("Help", help_response.response.rect);

                ui.separator();

                let pan_btn = ui.add_enabled(
                    current_tool != rgis_settings::Tool::Pan,
                    egui::Button::new("Pan")
                        .selected(current_tool == rgis_settings::Tool::Pan),
                );
                crate::widget_registry::register("Pan Tool", pan_btn.rect);
                if pan_btn.clicked() {
                    new_tool = rgis_settings::Tool::Pan;
                }

                let query_btn = ui.add_enabled(
                    current_tool != rgis_settings::Tool::Query,
                    egui::Button::new("Query")
                        .selected(current_tool == rgis_settings::Tool::Query),
                );
                crate::widget_registry::register("Query Tool", query_btn.rect);
                if query_btn.clicked() {
                    new_tool = rgis_settings::Tool::Query;
                }

                let measure_btn = ui.add_enabled(
                    current_tool != rgis_settings::Tool::Measure,
                    egui::Button::new("Measure")
                        .selected(current_tool == rgis_settings::Tool::Measure),
                );
                crate::widget_registry::register("Measure Tool", measure_btn.rect);
                if measure_btn.clicked() {
                    new_tool = rgis_settings::Tool::Measure;
                }

                if new_tool != current_tool {
                    actions.push(TopPanelAction::SetTool(new_tool));
                }
            });
        });

        self.top_panel_height.0 = inner_response.response.rect.height();
        actions
    }
}
