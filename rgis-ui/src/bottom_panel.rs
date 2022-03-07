use bevy_egui::egui;

pub(crate) struct BottomPanel<'a> {
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
//     pub state: &'a mut crate::UiState,
//     pub rgis_layers_resource: &'a rgis_layers::ArcLayers,
//     pub toggle_events: &'a mut bevy::app::Events<rgis_events::ToggleLayerVisibilityEvent>,
//     pub toggle_material_events: &'a mut bevy::app::Events<rgis_events::ToggleMaterialEvent>,
//     pub center_layer_events: &'a mut bevy::app::Events<rgis_events::CenterCameraEvent>,
//     pub thread_pool: &'a bevy::tasks::AsyncComputeTaskPool,
//     pub opened_file_bytes_sender: &'a crate::OpenedFileBytesSender,
//     pub mouse_pos: &'a rgis_mouse::MousePos,
}

impl<'a> BottomPanel<'a> {
    pub fn render(&mut self) {
        egui::TopBottomPanel::bottom("bottom").show(self.bevy_egui_ctx.ctx_mut(), |_ui| {
	});
        // egui::TopBottomPanel::top("top_panel").show(self.bevy_egui_ctx.ctx_mut(), |ui| {
        //     ui.horizontal(|ui| {
        //         ui.label("rgis");
        //         ui.menu_button("File", |ui| {
        //             render_exit_button(self.app_exit_events, ui);
        //         });
        //         ui.menu_button("View", |ui| {
        //             render_full_screen_button(self.windows, ui);
        //         });
        //         ui.menu_button("Help", |ui| {
        //             if ui.button("Source code").clicked() {
        //                 let _ = webbrowser::open("https://github.com/frewsxcv/rgis");
        //             }
        //         });
        //     });
        // });
    }
}
