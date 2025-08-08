use bevy_egui::egui;
use geodesy::Context;
use std::str::FromStr;

pub struct CrsInput<'a> {
    pub geodesy_ctx: &'a rgis_geodesy::GeodesyContext,
    pub text_field_value: &'a mut String,
    outcome: &'a mut Option<Outcome>,
}

impl<'a> CrsInput<'a> {
    pub fn new(
        geodesy_ctx: &'a rgis_geodesy::GeodesyContext,
        outcome: &'a mut Option<Outcome>,
        text_field_value: &'a mut String,
    ) -> Self {
        Self {
            geodesy_ctx,
            outcome,
            text_field_value,
        }
    }
}

pub type Outcome = Result<(geodesy::OpHandle, u16), Error>;

impl egui::Widget for CrsInput<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.add(EpsgCodeInputFieldWidget::new(
                self.geodesy_ctx,
                self.text_field_value,
                self.outcome,
            ));

            let Some(outcome) = self.outcome else { return };

            match outcome {
                Ok((op_handle, _)) => {
                    ui.vertical(|ui| {
                        if let Ok(geodesy_ctx) = self.geodesy_ctx.0.read() {
                            if let Ok(steps) = geodesy_ctx.steps(*op_handle) {
                                for step in steps {
                                    ui.label(egui::RichText::new(step).code());
                                }
                            }
                        } else {
                            ui.label(
                                egui::RichText::new("Failed to get read lock on geodesy context")
                                    .color(ui.visuals().error_fg_color),
                            );
                        }
                    });
                }
                Err(e) => {
                    ui.label(format!("{e}"));
                }
            }
        })
        .response
    }
}

struct EpsgCodeInputFieldWidget<'a> {
    geodesy_ctx: &'a rgis_geodesy::GeodesyContext,
    text_field_value: &'a mut String,
    outcome: &'a mut Option<Outcome>,
    parsed_text_field_value: Option<u16>,
}

impl<'a> EpsgCodeInputFieldWidget<'a> {
    pub fn new(
        geodesy_ctx: &'a rgis_geodesy::GeodesyContext,
        text_field_value: &'a mut String,
        outcome: &'a mut Option<Outcome>,
    ) -> Self {
        Self {
            geodesy_ctx,
            text_field_value,
            outcome,
            parsed_text_field_value: None,
        }
    }
}

impl egui::Widget for EpsgCodeInputFieldWidget<'_> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label("EPSG:");
            let edit_field = ui.text_edit_singleline(self.text_field_value);

            *self.outcome = if edit_field.changed()
                || (!self.text_field_value.is_empty() && self.outcome.is_none())
            {
                ui.add(ValidIconWidget);
                Some(parse_epsg_input_value(
                    self.geodesy_ctx,
                    self.text_field_value,
                    &mut self.parsed_text_field_value,
                ))
            } else if let Some(n) = self.outcome.take() {
                if n.is_ok() {
                    ui.add(ValidIconWidget);
                } else {
                    ui.add(InvalidIconWidget);
                }
                Some(n)
            } else {
                ui.add(InvalidIconWidget);
                None
            };
        })
        .response
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Geodesy(#[from] rgis_geodesy::Error),
    #[error("RwLock poisoned")]
    RwLock,
}

fn parse_epsg_input_value(
    geodesy_ctx: &rgis_geodesy::GeodesyContext,
    input: &str,
    parsed_text_field_value: &mut Option<u16>,
) -> Outcome {
    let mut geodesy_ctx = geodesy_ctx.0.write().map_err(|_| Error::RwLock)?;
    let parsed = u16::from_str(input);
    *parsed_text_field_value = parsed.as_ref().ok().copied();
    let parsed = parsed?;
    let outcome = rgis_geodesy::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx, parsed)
        .map_err(Error::Geodesy)?;
    Ok((outcome, parsed))
}

struct ValidIconWidget;

impl egui::Widget for ValidIconWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label(egui::RichText::new("✅"))
    }
}

struct InvalidIconWidget;

impl egui::Widget for InvalidIconWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label(egui::RichText::new("❌").color(ui.visuals().error_fg_color))
    }
}
