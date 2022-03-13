use bevy_egui::egui;

pub(crate) struct AddLayerWindow<'a> {
    pub state: &'a mut crate::UiState,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub thread_pool: &'a bevy::tasks::AsyncComputeTaskPool,
    pub opened_file_bytes_sender: &'a crate::OpenedFileBytesSender,
}

impl<'a> AddLayerWindow<'a> {
    pub fn render(&mut self) {
        egui::Window::new("Add Layer")
            .open(&mut self.state.is_add_layer_window_visible)
            .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                if ui.button("Add GeoJSON Layer").clicked() {
                    open_geojson_layer(self.opened_file_bytes_sender, self.thread_pool)
                }
                ui.separator();
                if ui.button("Add US Borders Layer").clicked() {
                    let sender = self.opened_file_bytes_sender.clone();
                    self.thread_pool
                        .spawn(async move {
                            sender
                                .send((
                                    rgis_library::ENTRIES[0].name.to_string(),
                                    rgis_library::ENTRIES[0].bytes.to_owned(),
                                ))
                                .await
                                .unwrap();
                        })
                        .detach();
                }
            });
    }
}

fn open_geojson_layer(
    opened_file_bytes_sender: &crate::OpenedFileBytesSender,
    thread_pool: &bevy::tasks::AsyncComputeTaskPool,
) {
    let sender = opened_file_bytes_sender.clone();
    thread_pool
        .spawn(async move {
            let task = rfd::AsyncFileDialog::new().pick_file();
            let file_handle = task.await;
            if let Some(n) = file_handle {
                sender.send((n.file_name(), n.read().await)).await.unwrap();
            }
        })
        .detach();
}
