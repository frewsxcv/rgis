use bevy::log::error;
use bevy_egui::egui;
use geo_file_loader::FileFormat;

use super::{file::SelectedFile, AddLayerOutput, State};

pub struct ByFile<'a> {
    pub selected_file: &'a mut SelectedFile,
    pub state: &'a mut State,
}

impl<'a> ByFile<'a> {
    pub fn show(self, ui: &mut egui::Ui) -> Option<AddLayerOutput> {
        let mut output = None;

        ui.label("Format:");

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

        ui.radio_value(
            &mut self.state.selected_format,
            Some(FileFormat::Shapefile),
            "Shapefile",
        );

        ui.radio_value(
            &mut self.state.selected_format,
            Some(FileFormat::Wkt),
            "WKT",
        );

        let Some(selected_format) = self.state.selected_format else {
            return None;
        };

        ui.separator();

        ui.label("Select file:");

        if ui.button("📄 Select file").clicked() {
            output = Some(AddLayerOutput::OpenFile);
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
                    output = Some(AddLayerOutput::LoadFromFile {
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
                    error!("Expected file to exist when loading, but no file exists");
                }
            };
        }

        output
    }
}
