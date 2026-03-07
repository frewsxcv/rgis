use bevy::prelude::*;
use bevy_egui::egui;
use rgis_events::ChangeCrsMessage;

pub struct ChangeCrs<'a, 'w> {
    pub text_field_value: &'a mut String,
    pub crs_input_mode: &'a mut crate::widgets::crs_input::CrsInputMode,
    pub change_crs_event_writer: &'a mut MessageWriter<'w, ChangeCrsMessage>,
    pub target_crs: rgis_crs::TargetCrs,
    pub crs_input_outcome: &'a mut Option<crate::widgets::crs_input::Outcome>,
    pub geodesy_ctx: &'a rgis_crs::GeodesyContext,
}

impl ChangeCrs<'_, '_> {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.label("Common projections:");
        ui.horizontal(|ui| {
            if ui.button("WGS 84 (4326)").clicked() {
                *self.text_field_value = "4326".into();
            }
            if ui.button("Web Mercator (3857)").clicked() {
                *self.text_field_value = "3857".into();
            }
        });
        ui.add_space(4.0);
        ui.add(crate::widgets::crs_input::CrsInput::new(
            self.geodesy_ctx,
            self.crs_input_outcome,
            self.text_field_value,
            self.crs_input_mode,
        ));
        let button = egui::Button::new("Set");
        match self.crs_input_outcome {
            Some(Ok((op_handle, epsg_code, proj_string))) => {
                if ui.add_enabled(true, button).clicked() {
                    self.change_crs_event_writer.write(ChangeCrsMessage {
                        old: self.target_crs.0.clone(),
                        new: rgis_primitives::Crs {
                            epsg_code: *epsg_code,
                            proj_string: proj_string.clone(),
                            op_handle: *op_handle,
                        },
                    });
                }
            }
            _ => {
                ui.add_enabled(false, button);
            }
        };
    }
}
