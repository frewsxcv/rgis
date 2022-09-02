use bevy_egui::egui;
use std::mem;

#[derive(bevy::ecs::system::SystemParam)]
pub struct Events<'w, 's> {
    pub load_geo_json_file_event_writer: bevy::ecs::event::EventWriter<
        'w,
        's,
        rgis_events::LoadFileEvent<geo_file_loader::GeoJsonSource>,
    >,
    pub show_add_layer_window_event_reader:
        bevy::ecs::event::EventReader<'w, 's, rgis_events::ShowAddLayerWindow>,
    pub hide_add_layer_window_events:
        bevy::ecs::system::ResMut<'w, bevy::ecs::event::Events<rgis_events::HideAddLayerWindow>>,
}

pub struct OpenFileTask;

impl bevy_jobs::Job for OpenFileTask {
    type Outcome = Option<OpenedFile>;

    fn name(&self) -> String {
        "Opening file".into()
    }

    fn perform(self) -> bevy_jobs::AsyncReturn<Self::Outcome> {
        Box::pin(async move {
            let task = rfd::AsyncFileDialog::new().pick_file();
            let file_handle = task.await?;
            let file_name = file_handle.file_name();
            let bytes = file_handle.read().await;
            Some(OpenedFile { file_name, bytes })
        })
    }
}

pub(crate) struct AddLayerWindow<'a, 'w1, 's1, 'w2, 's2> {
    pub state: &'a mut State,
    pub is_visible: &'a mut bool,
    pub selected_file: &'a mut SelectedFile,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub task_spawner: &'a mut bevy_jobs::JobSpawner<'w1, 's1>,
    pub events: &'a mut Events<'w2, 's2>,
}

#[derive(PartialEq, Eq, Default)]
enum Source {
    #[default]
    Unselected,
    Library,
    File,
    Text,
}

#[derive(Default)]
pub struct State {
    pub text_edit_contents: String,
    selected_source: Source,
    selected_format: Format,
}

#[derive(Default)]
pub struct SelectedFile(pub Option<OpenedFile>);

impl State {
    pub fn reset(&mut self) {
        self.text_edit_contents = String::new();
        self.selected_source = Source::Unselected;
        self.selected_format = Format::Unselected;
    }
}

pub struct OpenedFile {
    bytes: Vec<u8>,
    file_name: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Format {
    #[default]
    Unselected,
    GeoJson,
    Wkt,
}

impl<'a, 'w1, 's1, 'w2, 's2> AddLayerWindow<'a, 'w1, 's1, 'w2, 's2> {
    pub(crate) fn render(&mut self) {
        if !*self.is_visible {
            return;
        }

        egui::Window::new("Add Layer")
            .open(self.is_visible)
            .anchor(egui::Align2::LEFT_TOP, [5., 5.])
            .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                ui.label("Layer source:");

                ui.radio_value(&mut self.state.selected_source, Source::Library, "Library");
                ui.radio_value(&mut self.state.selected_source, Source::File, "File");
                ui.radio_value(&mut self.state.selected_source, Source::Text, "Text");

                if self.state.selected_source == Source::Unselected {
                    return;
                }

                ui.separator();

                if self.state.selected_source == Source::Library {
                    for entry in rgis_library::ENTRIES {
                        if ui.button(format!("Add '{}' Layer", entry.name)).clicked() {
                            self.events.load_geo_json_file_event_writer.send(
                                rgis_events::LoadFileEvent::FromNetwork {
                                    name: entry.name.into(),
                                    url: entry.url.into(),
                                    crs: entry.crs.into(),
                                },
                            );
                            self.events.hide_add_layer_window_events.send_default();
                        }
                    }
                    return;
                }

                if self.state.selected_source == Source::File
                    || self.state.selected_source == Source::Text
                {
                    ui.label("Format:");

                    ui.radio_value(&mut self.state.selected_format, Format::GeoJson, "GeoJSON");
                    ui.radio_value(&mut self.state.selected_format, Format::Wkt, "WKT");
                }

                if self.state.selected_format == Format::Unselected {
                    return;
                }

                ui.separator();

                if self.state.selected_source == Source::File {
                    ui.label("Select file:");

                    if ui.button("📄 Select file").clicked() {
                        self.task_spawner.spawn(OpenFileTask);
                    }

                    let submittable = self.selected_file.0.is_some();

                    if let Some(loaded_file) = &self.selected_file.0 {
                        ui.label(format!("Selected file: {}", loaded_file.file_name));
                    }

                    ui.separator();

                    if ui
                        .add_enabled(submittable, egui::Button::new("Add layer"))
                        .clicked()
                    {
                        match self.state.selected_format {
                            Format::GeoJson => {}
                            Format::Wkt => {}
                            Format::Unselected => {}
                        }
                        let loaded_file = self.selected_file.0.take().unwrap();
                        self.events.load_geo_json_file_event_writer.send(
                            rgis_events::LoadFileEvent::FromBytes {
                                file_name: loaded_file.file_name,
                                file_loader: geo_file_loader::GeoJsonSource {
                                    bytes: loaded_file.bytes,
                                },
                                crs: "EPSG:4326".into(),
                            },
                        );
                        self.events.hide_add_layer_window_events.send_default();
                        self.state.reset();
                    }
                } else if self.state.selected_source == Source::Text {
                    ui.label("Input text:");

                    egui::ScrollArea::vertical()
                        .max_height(300.)
                        .show(ui, |ui| {
                            egui::widgets::TextEdit::multiline(&mut self.state.text_edit_contents)
                                .code_editor()
                                .hint_text(hint_text(self.state.selected_format))
                                .show(ui);
                        });

                    let submittable = !self.state.text_edit_contents.is_empty();

                    ui.separator();

                    if ui
                        .add_enabled(submittable, egui::Button::new("Add layer"))
                        .clicked()
                    {
                        let new = mem::take(&mut self.state.text_edit_contents);
                        self.events.load_geo_json_file_event_writer.send(
                            rgis_events::LoadFileEvent::FromBytes {
                                file_name: "Inputted file".into(),
                                file_loader: geo_file_loader::GeoJsonSource {
                                    bytes: new.into_bytes(),
                                },
                                crs: "EPSG:4326".into(),
                            },
                        );
                        self.events.hide_add_layer_window_events.send_default();
                        self.state.reset();
                    }
                }
            });

        // If the user closes the window, reset the state.
        if !*self.is_visible {
            self.state.reset();
        }
    }
}

fn hint_text(format: Format) -> &'static str {
    match format {
        Format::Unselected => "",
        Format::GeoJson => "{\n  \"type\": \"FeatureCollection\",\n  \"features\": []\n}",
        Format::Wkt => "LINESTRING (30 10, 10 30, 40 40)",
    }
}
