use bevy::prelude::*;
use bevy_egui::egui;
use geo_file_loader::FileFormat;
use std::mem;

#[derive(bevy::ecs::system::SystemParam)]
pub struct Events<'w, 's> {
    pub load_file_event_writer: bevy::ecs::event::EventWriter<'w, rgis_events::LoadFileEvent>,
    pub show_add_layer_window_event_reader:
        bevy::ecs::event::EventReader<'w, 's, rgis_events::ShowAddLayerWindow>,
    pub hide_add_layer_window_events:
        bevy::ecs::system::ResMut<'w, bevy::ecs::event::Events<rgis_events::HideAddLayerWindow>>,
}

pub struct OpenFileJob;

impl bevy_jobs::Job for OpenFileJob {
    type Outcome = Option<OpenedFile>;
    const JOB_TYPE: bevy_jobs::JobType = bevy_jobs::JobType::Io;

    fn name(&self) -> String {
        "Opening file".into()
    }

    async fn perform(self, _: bevy_jobs::Context) -> Self::Outcome {
        let task = rfd::AsyncFileDialog::new().pick_file();
        let file_handle = task.await?;
        let file_name = file_handle.file_name();
        let bytes = file_handle.read().await;
        Some(OpenedFile { file_name, bytes })
    }
}

pub(crate) struct AddLayerWindow<'a> {
    pub state: &'a mut State,
    pub is_visible: &'a mut bool,
    pub selected_file: &'a mut SelectedFile,
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub geodesy_ctx: &'a rgis_geodesy::GeodesyContext,
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

pub enum AddLayerWindowOutput {
    LoadFromText {
        text: String,
        file_format: FileFormat,
        source_crs: rgis_primitives::Crs,
    },
    LoadFromFile {
        file_name: String,
        file_format: FileFormat,
        bytes: Vec<u8>,
        source_crs: rgis_primitives::Crs,
    },
    LoadFromLibrary {
        name: String,
        url: String,
        source_crs: rgis_primitives::Crs,
    },
    OpenFile,
}

impl AddLayerWindow<'_> {
    pub(crate) fn render(&mut self) -> Option<AddLayerWindowOutput> {
        let mut output = None;

        if *self.is_visible {
            egui::Window::new("Add Layer")
                .resizable(false)
                .open(self.is_visible)
                .show(self.egui_ctx, |ui| {
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
                        if let Some(new_output) = (LibraryWidget {
                            geodesy_ctx: self.geodesy_ctx,
                        })
                        .show(ui)
                        {
                            output = Some(new_output);
                        }
                        return;
                    }

                    ui.label("Source CRS:");
                    let crs_input_widget = crate::widgets::crs_input::CrsInput::new(
                        self.geodesy_ctx,
                        &mut self.state.crs_input_outcome,
                        &mut self.state.crs_input,
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
                        ui.radio_value(
                            &mut self.state.selected_format,
                            Some(FileFormat::Wkt),
                            "WKT",
                        );
                    }

                    let Some(selected_format) = self.state.selected_format else {
                        return;
                    };

                    ui.separator();

                    if self.state.selected_source == Source::File {
                        ui.label("Select file:");

                        if ui.button("📄 Select file").clicked() {
                            output = Some(AddLayerWindowOutput::OpenFile);
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
                                Some(loaded_file) => {
                                    output = Some(AddLayerWindowOutput::LoadFromFile {
                                        file_name: loaded_file.file_name,
                                        file_format: selected_format,
                                        bytes: loaded_file.bytes,
                                        source_crs: rgis_primitives::Crs {
                                            epsg_code: self
                                                .state
                                                .crs_input_outcome
                                                .as_ref()
                                                .unwrap()
                                                .as_ref()
                                                .unwrap()
                                                .1,
                                            op_handle: self
                                                .state
                                                .crs_input_outcome
                                                .as_ref()
                                                .unwrap()
                                                .as_ref()
                                                .unwrap()
                                                .0,
                                        },
                                    });
                                }
                                None => {
                                    bevy::log::error!(
                                        "Expected file to exist when loading, but no file exists"
                                    );
                                }
                            };
                        }
                    } else if self.state.selected_source == Source::Text {
                        ui.label("Input text:");

                        egui::ScrollArea::vertical()
                            .max_height(300.)
                            .show(ui, |ui| {
                                egui::widgets::TextEdit::multiline(
                                    &mut self.state.text_edit_contents,
                                )
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
                                FileFormat::Shapefile => {
                                    unreachable!()
                                }
                                file_format @ (FileFormat::Wkt
                                | FileFormat::GeoJson
                                | FileFormat::Gpx) => {
                                    output = Some(AddLayerWindowOutput::LoadFromText {
                                        text: new,
                                        file_format,
                                        source_crs: rgis_primitives::Crs {
                                            epsg_code: self
                                                .state
                                                .crs_input_outcome
                                                .as_ref()
                                                .unwrap()
                                                .as_ref()
                                                .unwrap()
                                                .1,
                                            op_handle: self
                                                .state
                                                .crs_input_outcome
                                                .as_ref()
                                                .unwrap()
                                                .as_ref()
                                                .unwrap()
                                                .0,
                                        },
                                    });
                                }
                            }
                        }
                    }
                });
        } else {
            // If the user closes the window, reset the state.
            self.state.reset();
        }

        output
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

struct LibraryWidget<'a> {
    geodesy_ctx: &'a rgis_geodesy::GeodesyContext,
}

impl LibraryWidget<'_> {
    fn show(self, ui: &mut egui::Ui) -> Option<AddLayerWindowOutput> {
        let mut output = None;
        ui.vertical(|ui| {
            ui.heading("Library");
            for folder in rgis_library::get() {
                ui.collapsing(format!("📁 {}", folder.name), |ui| {
                    for entry in &folder.entries {
                        if let Some(new_output) = (LibraryEntryWidget {
                            folder,
                            entry,
                            geodesy_ctx: self.geodesy_ctx,
                        })
                        .show(ui)
                        {
                            output = Some(new_output);
                        }
                    }
                });
            }
        });
        output
    }
}

struct LibraryEntryWidget<'a> {
    entry: &'a rgis_library::Entry,
    folder: &'a rgis_library::Folder,
    geodesy_ctx: &'a rgis_geodesy::GeodesyContext,
}

impl LibraryEntryWidget<'_> {
    fn show(self, ui: &mut egui::Ui) -> Option<AddLayerWindowOutput> {
        let mut output = None;
        ui.horizontal(|ui| {
            if ui.button("➕ Add").clicked() {
                let mut geodesy_ctx = self.geodesy_ctx.0.write().unwrap();
                let op_handle =
                    rgis_geodesy::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx, self.entry.crs)
                        .unwrap();
                output = Some(AddLayerWindowOutput::LoadFromLibrary {
                    name: format!("{}: {}", self.folder.name, self.entry.name),
                    url: self.entry.url.into(),
                    source_crs: rgis_primitives::Crs {
                        epsg_code: self.entry.crs,
                        op_handle,
                    },
                });
            }
            ui.label(self.entry.name);
        });
        output
    }
}
