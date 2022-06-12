use bevy_egui::egui;
use rgis_task::Task;

pub struct OpenFileTask;

impl rgis_task::Task for OpenFileTask {
    type Outcome = Option<(String, Vec<u8>)>;

    fn name(&self) -> String {
        "Opening file".into()
    }

    fn perform(self) -> rgis_task::PerformReturn<Self::Outcome> {
        Box::pin(async move {
            let task = rfd::AsyncFileDialog::new().pick_file();
            let file_handle = task.await?;
            let file_name = file_handle.file_name();
            let bytes = file_handle.read().await;
            Some((file_name, bytes))
        })
    }
}

pub(crate) struct AddLayerWindow<'a, 'w1, 's1, 'w2, 's2> {
    pub state: &'a mut crate::UiState,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub thread_pool: &'a bevy::tasks::AsyncComputeTaskPool,
    pub load_geo_json_file_event_writer:
        &'a mut bevy::ecs::event::EventWriter<'w1, 's1, rgis_events::LoadGeoJsonFileEvent>,
    pub commands: &'a mut bevy::prelude::Commands<'w2, 's2>,
}

impl<'a, 'w1, 's1, 'w2, 's2> AddLayerWindow<'a, 'w1, 's1, 'w2, 's2> {
    pub(crate) fn render(&mut self) {
        egui::Window::new("Add Layer")
            .open(&mut self.state.is_add_layer_window_visible)
            .anchor(egui::Align2::LEFT_TOP, [5., 5.])
            .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                if ui.button("Add GeoJSON Layer").clicked() {
                    OpenFileTask.spawn(self.thread_pool, self.commands)
                }
                ui.separator();
                for entry in rgis_library::ENTRIES {
                    if ui.button(format!("Add '{}' Layer", entry.name)).clicked() {
                        // TODO: this should happen in a background task
                        self.load_geo_json_file_event_writer.send(
                            rgis_events::LoadGeoJsonFileEvent::FromNetwork {
                                name: entry.name.into(),
                                url: entry.url.into(),
                                crs: entry.crs.into(),
                            },
                        )
                    }
                }
            });
    }
}
