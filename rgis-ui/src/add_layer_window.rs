use bevy::prelude::*;
use bevy_egui::egui;
use std::mem;
use std::str::FromStr;
use geo_file_loader::FileFormat;

#[derive(bevy::ecs::system::SystemParam)]
pub struct Events<'w, 's> {
    pub load_geo_json_file_event_writer: bevy::ecs::event::EventWriter<
        'w,
        rgis_events::LoadFileEvent<geo_file_loader::GeoJsonSource>,
    >,
    pub load_wkt_file_event_writer:
        bevy::ecs::event::EventWriter<'w, rgis_events::LoadFileEvent<geo_file_loader::WktSource>>,
    pub load_gpx_file_event_writer:
        bevy::ecs::event::EventWriter<'w, rgis_events::LoadFileEvent<geo_file_loader::GpxSource>>,
    pub load_shapefile_file_event_writer: bevy::ecs::event::EventWriter<
        'w,
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
    selected_format: Option<FileFormat>,
    crs_input_outcome: Option<crate::widgets::crs_input::Outcome>,
}

const DEFAULT_CRS_INPUT: &str = "4326";

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

impl<'a, 'w1, 's1, 'w2, 's2> AddLayerWindow<'a, 'w1, 's1, 'w2, 's2> {
    pub(crate) fn render(&mut self) {
        if !*self.is_visible {
            return;
        }

        egui::Window::new("Add Layer")
            .open(self.is_visible)
            .anchor(egui::Align2::LEFT_TOP, [5., 5.])
            .show(self.bevy_egui_ctx.get_mut(), |ui| {
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
                    ui.add(LibraryWidget {
                        events: self.events,
                    });
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
                        Some(FileFormat::GeoJson),
                        "GeoJSON",
                    );

                    ui.radio_value(
                        &mut self.state.selected_format,
                        Some(FileFormat::Gpx),
                        "GPX",
                    );
                }

                if self.state.selected_source == Source::File {
                    ui.radio_value(
                        &mut self.state.selected_format,
                        Some(FileFormat::Shapefile),
                        "Shapefile",
                    );
                }

                if self.state.selected_source == Source::File
                    || self.state.selected_source == Source::Text
                {
                    ui.radio_value(&mut self.state.selected_format, Some(FileFormat::Wkt), "WKT");
                }

                let Some(selected_format) = self.state.selected_format else {
                    return;
                };

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
                                FileFormat::GeoJson => {
                                    self.events.load_geo_json_file_event_writer.send(
                                        rgis_events::LoadFileEvent::FromBytes {
                                            file_name: loaded_file.file_name,
                                            file_loader: geo_file_loader::GeoJsonSource {
                                                bytes: loaded_file.bytes.into(),
                                            },
                                            crs_epsg_code: 4326,
                                        },
                                    );
                                }
                                FileFormat::Shapefile => {
                                    self.events.load_shapefile_file_event_writer.send(
                                        rgis_events::LoadFileEvent::FromBytes {
                                            file_name: loaded_file.file_name,
                                            file_loader: geo_file_loader::ShapefileSource {
                                                bytes: loaded_file.bytes.into(),
                                            },
                                            // TODO: don't allow the user to add a layer if the CRS isn't valid
                                            crs_epsg_code: u16::from_str(&self.state.crs_input)
                                                .unwrap(),
                                        },
                                    );
                                }
                                FileFormat::Wkt => {
                                    self.events.load_wkt_file_event_writer.send(
                                        rgis_events::LoadFileEvent::FromBytes {
                                            file_name: loaded_file.file_name,
                                            file_loader: geo_file_loader::WktSource {
                                                bytes: loaded_file.bytes.into(),
                                            },
                                            // TODO: don't allow the user to add a layer if the CRS isn't valid
                                            crs_epsg_code: u16::from_str(&self.state.crs_input)
                                                .unwrap(),
                                        },
                                    );
                                }
                                FileFormat::Gpx => {
                                    self.events.load_gpx_file_event_writer.send(
                                        rgis_events::LoadFileEvent::FromBytes {
                                            file_name: loaded_file.file_name,
                                            file_loader: geo_file_loader::GpxSource {
                                                bytes: loaded_file.bytes.into(),
                                            },
                                            // TODO: don't allow the user to add a layer if the CRS isn't valid
                                            crs_epsg_code: u16::from_str(&self.state.crs_input)
                                                .unwrap(),
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
                            FileFormat::GeoJson => {
                                self.events.load_geo_json_file_event_writer.send(
                                    rgis_events::LoadFileEvent::FromBytes {
                                        file_name: "Inputted file".into(),
                                        file_loader: geo_file_loader::GeoJsonSource {
                                            bytes: new.into(),
                                        },
                                        // TODO: don't allow the user to add a layer if the CRS isn't valid
                                        crs_epsg_code: u16::from_str(&self.state.crs_input)
                                            .unwrap(),
                                    },
                                );
                            }
                            FileFormat::Shapefile => {
                                unreachable!()
                            }
                            FileFormat::Wkt => {
                                self.events.load_wkt_file_event_writer.send(
                                    rgis_events::LoadFileEvent::FromBytes {
                                        file_name: "Inputted file".into(),
                                        file_loader: geo_file_loader::WktSource {
                                            bytes: new.into(),
                                        },
                                        // TODO: don't allow the user to add a layer if the CRS isn't valid
                                        crs_epsg_code: u16::from_str(&self.state.crs_input)
                                            .unwrap(),
                                    },
                                );
                            }
                            FileFormat::Gpx => {
                                self.events.load_gpx_file_event_writer.send(
                                    rgis_events::LoadFileEvent::FromBytes {
                                        file_name: "Inputted file".into(),
                                        file_loader: geo_file_loader::GpxSource {
                                            bytes: new.into(),
                                        },
                                        // TODO: don't allow the user to add a layer if the CRS isn't valid
                                        crs_epsg_code: u16::from_str(&self.state.crs_input)
                                            .unwrap(),
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

const fn hint_text(format: FileFormat) -> &'static str {
    match format {
        FileFormat::GeoJson => "{\n  \"type\": \"FeatureCollection\",\n  \"features\": []\n}",
        FileFormat::Shapefile => panic!("Shapefiles are not textual"),
        FileFormat::Wkt => "LINESTRING (30 10, 10 30, 40 40)",
        FileFormat::Gpx => "", // TODO: add example GPX
    }
}

struct LibraryWidget<'a, 'w, 's> {
    events: &'a mut Events<'w, 's>,
}

impl<'a, 'w, 's> egui::Widget for LibraryWidget<'a, 'w, 's> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.heading("Library");
            for folder in rgis_library::get() {
                ui.collapsing(format!("📁 {}", folder.name), |ui| {
                    for entry in &folder.entries {
                        ui.add(LibraryEntryWidget {
                            folder,
                            entry,
                            events: self.events,
                        });
                    }
                });
            }
        })
        .response
    }
}

struct LibraryEntryWidget<'a, 'w, 's> {
    entry: &'a rgis_library::Entry,
    folder: &'a rgis_library::Folder,
    events: &'a mut Events<'w, 's>,
}

impl<'a, 'w, 's> egui::Widget for LibraryEntryWidget<'a, 'w, 's> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            if ui.button("➕ Add").clicked() {
                self.events.load_geo_json_file_event_writer.send(
                    rgis_events::LoadFileEvent::FromNetwork {
                        name: format!("{}: {}", self.folder.name, self.entry.name),
                        url: self.entry.url.into(),
                        crs_epsg_code: self.entry.crs,
                    },
                );
                self.events.hide_add_layer_window_events.send_default();
            }
            ui.label(self.entry.name);
        })
        .response
    }
}
