use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::egui;
use geo_file_loader::FileFormat;
use rgis_file_loader_events::LoadFileEvent;
use rgis_ui_events::{HideAddLayerWindow, ShowAddLayerWindow};
use std::mem;

#[derive(SystemParam)]
pub struct Events<'w, 's> {
    pub load_file_event_writer: EventWriter<'w, LoadFileEvent>,
    pub show_add_layer_window_event_reader: EventReader<'w, 's, ShowAddLayerWindow>,
    pub hide_add_layer_window_events: ResMut<'w, bevy::prelude::Events<HideAddLayerWindow>>,
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

pub struct AddLayer<'a> {
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
    pub crs_input: String,
    selected_source: Source,
    pub selected_format: Option<FileFormat>,
    pub crs_input_outcome: Option<crate::widgets::crs_input::Outcome>,
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

pub enum AddLayerOutput {
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

impl AddLayer<'_> {
    pub fn render(&mut self) -> Option<AddLayerOutput> {
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

                    let crs_is_valid = self
                        .state
                        .crs_input_outcome
                        .as_ref()
                        .map(|o| o.is_ok())
                        .unwrap_or(false);

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
                            output = Some(AddLayerOutput::OpenFile);
                        }

                        let submittable = self.selected_file.0.is_some() && crs_is_valid;

                        if let Some(loaded_file) = &self.selected_file.0 {
                            ui.label(format!("Selected file: {}", loaded_file.file_name));
                        }

                        ui.separator();

                        if ui
                            .add_enabled(submittable, egui::Button::new("Add layer"))
                            .clicked()
                        {
                            if let (Some(loaded_file), Some(Ok((op_handle, epsg_code)))) = (
                                self.selected_file.0.take(),
                                self.state.crs_input_outcome.as_ref(),
                            ) {
                                output = Some(AddLayerOutput::LoadFromFile {
                                    file_name: loaded_file.file_name,
                                    file_format: selected_format,
                                    bytes: loaded_file.bytes,
                                    source_crs: rgis_primitives::Crs {
                                        epsg_code: *epsg_code,
                                        op_handle: *op_handle,
                                    },
                                });
                            } else {
                                error!(
                                    "Expected file to exist when loading, but no file exists"
                                );
                            }
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

                        let submittable =
                            !self.state.text_edit_contents.is_empty() && crs_is_valid;

                        ui.separator();

                        if ui
                            .add_enabled(submittable, egui::Button::new("Add layer"))
                            .clicked()
                        {
                            if let Some(Ok((op_handle, epsg_code))) =
                                self.state.crs_input_outcome.as_ref()
                            {
                                let new = mem::take(&mut self.state.text_edit_contents);
                                match selected_format {
                                    FileFormat::Shapefile => {
                                        unreachable!()
                                    }
                                    file_format @ (FileFormat::Wkt
                                    | FileFormat::GeoJson
                                    | FileFormat::Gpx) => {
                                        output = Some(AddLayerOutput::LoadFromText {
                                            text: new,
                                            file_format,
                                            source_crs: rgis_primitives::Crs {
                                                epsg_code: *epsg_code,
                                                op_handle: *op_handle,
                                            },
                                        });
                                    }
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
    fn show(self, ui: &mut egui::Ui) -> Option<AddLayerOutput> {
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
    fn show(self, ui: &mut egui::Ui) -> Option<AddLayerOutput> {
        let mut output = None;
        ui.horizontal(|ui| {
            if ui.button("➕ Add").clicked() {
                match self.geodesy_ctx.0.write() {
                    Ok(mut geodesy_ctx) => {
                        match rgis_geodesy::epsg_code_to_geodesy_op_handle(
                            &mut *geodesy_ctx,
                            self.entry.crs,
                        ) {
                            Ok(op_handle) => {
                                output = Some(AddLayerOutput::LoadFromLibrary {
                                    name: format!("{}: {}", self.folder.name, self.entry.name),
                                    url: self.entry.url.into(),
                                    source_crs: rgis_primitives::Crs {
                                        epsg_code: self.entry.crs,
                                        op_handle,
                                    },
                                });
                            }
                            Err(e) => {
                                error!(
                                    "Failed to get geodesy op handle for EPSG:{}: {:?}",
                                    self.entry.crs, e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to acquire write lock on geodesy context: {}", e);
                    }
                }
            }
            ui.label(self.entry.name);
        });
        output
    }
}
