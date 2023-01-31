use bevy::prelude::*;
use bevy_egui::egui;
use std::mem;

#[derive(bevy::ecs::system::SystemParam)]
pub struct Events<'w, 's> {
    pub load_geo_json_file_event_writer: bevy::ecs::event::EventWriter<
        'w,
        's,
        rgis_events::LoadFileEvent<geo_file_loader::GeoJsonSource>,
    >,
    pub load_wkt_file_event_writer: bevy::ecs::event::EventWriter<
        'w,
        's,
        rgis_events::LoadFileEvent<geo_file_loader::WktSource>,
    >,
    pub load_shapefile_file_event_writer: bevy::ecs::event::EventWriter<
        'w,
        's,
        rgis_events::LoadFileEvent<geo_file_loader::ShapefileSource>,
    >,
    pub show_add_layer_window_event_reader:
        bevy::ecs::event::EventReader<'w, 's, rgis_events::ShowAddLayerWindow>,
    pub hide_add_layer_window_events:
        bevy::ecs::system::ResMut<'w, bevy::ecs::event::Events<rgis_events::HideAddLayerWindow>>,
}

pub struct OpenFileJob;

impl bevy_jobs::Job for OpenFileJob {
    type Outcome = Option<OpenedFile>;

    fn name(&self) -> String {
        "Opening file".into()
    }

    fn perform(self, _: bevy_jobs::Context) -> bevy_jobs::AsyncReturn<Self::Outcome> {
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
    pub job_spawner: &'a mut bevy_jobs::JobSpawner<'w1, 's1>,
    pub events: &'a mut Events<'w2, 's2>,
}

#[derive(PartialEq, Eq)]
enum Source {
    Unselected,
    Library,
    File,
    Text,
}

pub struct State {
    pub text_edit_contents: String,
    crs_input: String,
    selected_source: Source,
    selected_format: Option<Format>,
    crs_input_outcome: Option<crate::widgets::crs_input::Outcome>,
}

const DEFAULT_CRS_INPUT: &str = "EPSG:4326";

impl Default for State {
    fn default() -> Self {
        State {
            text_edit_contents: "".into(),
            crs_input: DEFAULT_CRS_INPUT.into(),
            crs_input_outcome: None,
            selected_format: None,
            selected_source: Source::Unselected,
        }
    }
}

#[derive(Default, Resource)]
pub struct SelectedFile(pub Option<OpenedFile>);

impl State {
    pub fn reset(&mut self) {
        self.text_edit_contents = String::new();
        self.crs_input = DEFAULT_CRS_INPUT.into();
        self.selected_source = Source::Unselected;
        self.selected_format = None;
    }
}

pub struct OpenedFile {
    bytes: Vec<u8>,
    file_name: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Format {
    GeoJson,
    Shapefile,
    Wkt,
}

impl Format {
    fn is_plaintext(self) -> bool {
        match self {
            Self::GeoJson => true,
            Self::Shapefile => false,
            Self::Wkt => true,
        }
    }
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

                // If the user switched to "Text" and and they don't have a plaintext format selected, unselect their selection
                if self.state.selected_source == Source::Text
                    && self
                        .state
                        .selected_format
                        .map(|f| !f.is_plaintext())
                        .unwrap_or(false)
                {
                    self.state.selected_format = None;
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

                ui.label("Source CRS:");
                let crs_input_widget = crate::widgets::CrsInput::new(
                    &mut self.state.crs_input,
                    &mut self.state.crs_input_outcome,
                );
                ui.add(crs_input_widget);

                ui.separator();

                if self.state.selected_source == Source::File
                    || self.state.selected_source == Source::Text
                {
                    ui.label("Format:");
                }

                if self.state.selected_source == Source::File
                    || self.state.selected_source == Source::Text
                {
                    ui.radio_value(
                        &mut self.state.selected_format,
                        Some(Format::GeoJson),
                        "GeoJSON",
                    );
                }

                if self.state.selected_source == Source::File {
                    ui.radio_value(
                        &mut self.state.selected_format,
                        Some(Format::Shapefile),
                        "Shapefile",
                    );
                }

                if self.state.selected_source == Source::File
                    || self.state.selected_source == Source::Text
                {
                    ui.radio_value(&mut self.state.selected_format, Some(Format::Wkt), "WKT");
                }

                let Some(selected_format) = self.state.selected_format else { return };

                ui.separator();

                if self.state.selected_source == Source::File {
                    ui.label("Select file:");

                    if ui.button("📄 Select file").clicked() {
                        self.job_spawner.spawn(OpenFileJob);
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
                        match self.selected_file.0.take() {
                            Some(loaded_file) => match selected_format {
                                Format::GeoJson => {
                                    self.events.load_geo_json_file_event_writer.send(
                                        rgis_events::LoadFileEvent::FromBytes {
                                            file_name: loaded_file.file_name,
                                            file_loader: geo_file_loader::GeoJsonSource {
                                                bytes: loaded_file.bytes.into(),
                                            },
                                            crs: "EPSG:4326".into(),
                                        },
                                    );
                                }
                                Format::Shapefile => {
                                    self.events.load_shapefile_file_event_writer.send(
                                        rgis_events::LoadFileEvent::FromBytes {
                                            file_name: loaded_file.file_name,
                                            file_loader: geo_file_loader::ShapefileSource {
                                                bytes: loaded_file.bytes.into(),
                                            },
                                            // TODO: don't allow the user to add a layer if the CRS isn't valid
                                            crs: self.state.crs_input.clone(),
                                        },
                                    );
                                }
                                Format::Wkt => {
                                    self.events.load_wkt_file_event_writer.send(
                                        rgis_events::LoadFileEvent::FromBytes {
                                            file_name: loaded_file.file_name,
                                            file_loader: geo_file_loader::WktSource {
                                                bytes: loaded_file.bytes.into(),
                                            },
                                            // TODO: don't allow the user to add a layer if the CRS isn't valid
                                            crs: self.state.crs_input.clone(),
                                        },
                                    );
                                }
                            },
                            None => {
                                bevy::log::error!(
                                    "Expected file to exist when loading, but no file exists"
                                );
                            }
                        };
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
                                .hint_text(hint_text(selected_format))
                                .show(ui);
                        });

                    let submittable = !self.state.text_edit_contents.is_empty();

                    ui.separator();

                    if ui
                        .add_enabled(submittable, egui::Button::new("Add layer"))
                        .clicked()
                    {
                        let new = mem::take(&mut self.state.text_edit_contents);
                        match selected_format {
                            Format::GeoJson => {
                                self.events.load_geo_json_file_event_writer.send(
                                    rgis_events::LoadFileEvent::FromBytes {
                                        file_name: "Inputted file".into(),
                                        file_loader: geo_file_loader::GeoJsonSource {
                                            bytes: new.into(),
                                        },
                                        // TODO: don't allow the user to add a layer if the CRS isn't valid
                                        crs: self.state.crs_input.clone(),
                                    },
                                );
                            }
                            Format::Shapefile => {
                                unreachable!()
                            }
                            Format::Wkt => {
                                self.events.load_wkt_file_event_writer.send(
                                    rgis_events::LoadFileEvent::FromBytes {
                                        file_name: "Inputted file".into(),
                                        file_loader: geo_file_loader::WktSource {
                                            bytes: new.into(),
                                        },
                                        // TODO: don't allow the user to add a layer if the CRS isn't valid
                                        crs: self.state.crs_input.clone(),
                                    },
                                );
                            }
                        }
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
        Format::GeoJson => "{\n  \"type\": \"FeatureCollection\",\n  \"features\": []\n}",
        Format::Shapefile => unreachable!("Shapefiles are not textual"),
        Format::Wkt => "LINESTRING (30 10, 10 30, 40 40)",
    }
}
