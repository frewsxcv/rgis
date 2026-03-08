use bevy::log::error;
use bevy_egui::egui;
use geo_file_loader::FileFormat;

use super::{file::SelectedFile, AddLayerOutput, State};

pub struct ByFile<'a> {
    pub selected_file: &'a mut SelectedFile,
    pub state: &'a mut State,
    pub geodesy_ctx: &'a rgis_crs::GeodesyContext,
}

impl<'a> ByFile<'a> {
    pub fn show(self, ui: &mut egui::Ui) -> Option<AddLayerOutput> {
        let mut output = None;

        ui.label("Format:");

        let geojson_radio = ui.radio_value(
            &mut self.state.selected_format,
            Some(FileFormat::GeoJson),
            "GeoJSON",
        );
        crate::widget_registry::register("GeoJSON", geojson_radio.rect);

        let gpx_radio = ui.radio_value(
            &mut self.state.selected_format,
            Some(FileFormat::Gpx),
            "GPX",
        );
        crate::widget_registry::register("GPX", gpx_radio.rect);

        let shapefile_radio = ui.radio_value(
            &mut self.state.selected_format,
            Some(FileFormat::Shapefile),
            "Shapefile",
        );
        crate::widget_registry::register("Shapefile", shapefile_radio.rect);

        let wkt_radio = ui.radio_value(
            &mut self.state.selected_format,
            Some(FileFormat::Wkt),
            "WKT",
        );
        crate::widget_registry::register("WKT", wkt_radio.rect);

        let geotiff_radio = ui.radio_value(
            &mut self.state.selected_format,
            Some(FileFormat::GeoTiff),
            "GeoTIFF",
        );
        crate::widget_registry::register("GeoTIFF", geotiff_radio.rect);

        let Some(selected_format) = self.state.selected_format else {
            return None;
        };

        ui.separator();

        ui.label("Select file:");

        let select_file_button = ui.button("📄 Select file");
        crate::widget_registry::register("Select file", select_file_button.rect);
        if select_file_button.clicked() {
            output = Some(AddLayerOutput::OpenFile);
        }

        let submittable = self.selected_file.0.is_some()
            && matches!(self.state.crs_input_outcome, Some(Ok(_)));

        if let Some(loaded_file) = &self.selected_file.0 {
            ui.label(format!("Selected file: {}", loaded_file.file_name));
        }

        ui.separator();

        ui.label("Source CRS:");
        ui.add(crate::widgets::crs_input::CrsInput::new(
            self.geodesy_ctx,
            &mut self.state.crs_input_outcome,
            &mut self.state.crs_input,
            &mut self.state.crs_input_mode,
            false,
        ));

        ui.separator();

        let add_layer_button = ui.add_enabled(submittable, egui::Button::new("Add layer"));
        crate::widget_registry::register("Add layer", add_layer_button.rect);
        if add_layer_button.clicked() {
            match self.selected_file.0.take() {
                Some(loaded_file) => {
                    let outcome = self
                        .state
                        .crs_input_outcome
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .unwrap();
                    output = Some(AddLayerOutput::LoadFromFile {
                        file_name: loaded_file.file_name,
                        file_format: selected_format,
                        bytes: loaded_file.bytes,
                        source_crs: rgis_primitives::Crs {
                            epsg_code: outcome.1,
                            proj_string: outcome.2.clone(),
                            op_handle: outcome.0,
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
