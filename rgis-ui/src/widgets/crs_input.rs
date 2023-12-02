use bevy_egui::egui;
use std::str::FromStr;
use transform::Context;

pub struct CrsInput<'a> {
    pub outcome: &'a mut Option<Outcome>,
    text_field_value: &'a mut String,
}

pub type Outcome = Result<(transform::Minimal, transform::OpHandle), Error>;

impl<'a> CrsInput<'a> {
    pub fn new(text_field_value: &'a mut String, prev_outcome: &'a mut Option<Outcome>) -> Self {
        CrsInput {
            outcome: prev_outcome,
            text_field_value,
        }
    }
}

impl<'a> egui::Widget for CrsInput<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.add(EpsgCodeInputFieldWidget {
                text_field_value: self.text_field_value,
                outcome: self.outcome,
            });

            let Some(outcome) = self.outcome else { return };

            match &outcome {
                Ok((ctx, op_handle)) => {
                    ui.vertical(|ui| {
                        let Ok(steps) = ctx.steps(*op_handle) else {
                            return;
                        };
                        for step in steps {
                            ui.label(egui::RichText::new(step).code());
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
    text_field_value: &'a mut String,
    outcome: &'a mut Option<Outcome>,
}

impl<'a> egui::Widget for EpsgCodeInputFieldWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label("EPSG:");
            let edit_field = ui.text_edit_singleline(self.text_field_value);

            *self.outcome = if edit_field.changed()
                || (!self.text_field_value.is_empty() && self.outcome.is_none())
            {
                ui.add(ValidIconWidget);
                Some(parse_epsg_input_value(self.text_field_value))
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
    TransformError(#[from] transform::Error),
}

fn parse_epsg_input_value(input: &str) -> Outcome {
    let parsed = u16::from_str(input)?;
    let outcome = transform::lookup_epsg_code(parsed)?;
    Ok(outcome)
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
