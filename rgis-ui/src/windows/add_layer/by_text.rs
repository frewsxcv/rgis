use std::mem;

use bevy_egui::egui;
use geo_file_loader::FileFormat;

use super::{AddLayerOutput, State};

pub struct ByText<'a> {
    pub state: &'a mut State,
}

impl<'a> ByText<'a> {
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
            Some(FileFormat::Wkt),
            "WKT",
        );

        let Some(selected_format) = self.state.selected_format else {
            return None;
        };

        ui.separator();

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
                FileFormat::Shapefile => {
                    unreachable!()
                }
                file_format @ (FileFormat::Wkt | FileFormat::GeoJson | FileFormat::Gpx) => {
                    output = Some(AddLayerOutput::LoadFromText {
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
