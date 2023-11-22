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
            let edit_field = ui
                .horizontal(|ui| {
                    ui.label("EPSG:");
                    ui.text_edit_singleline(self.text_field_value)
                })
                .inner;

            let outcome: Outcome = if edit_field.changed()
                || (!self.text_field_value.is_empty() && self.outcome.is_none())
            {
                parse_epsg_input_value(&self.text_field_value)
            } else if let Some(n) = self.outcome.take() {
                n
            } else {
                return;
            };

            let message = match &outcome {
                Ok((ctx, op_handle)) => format!("✅ {:?}", ctx.steps(*op_handle)),
                Err(e) => format!("❌ {e:?}"),
            };
            self.outcome.replace(outcome);
            ui.label(message);
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
