use bevy_egui::egui;
use proj4rs::Proj;
use std::str::FromStr;

pub struct CrsInput<'a> {
    pub outcome: &'a mut Option<Outcome>,
    text_field_value: &'a mut String,
}

pub type Outcome = Result<Proj, Box<dyn std::error::Error + Send + Sync>>;

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
                match u16::from_str(self.text_field_value) {
                    Ok(parsed) => proj4rs::Proj::from_epsg_code(parsed).map_err(|e| Box::new(e).into()),
                    Err(e) => Err(Box::new(e)),
                }
            } else if let Some(n) = self.outcome.take() {
                n
            } else {
                return;
            };

            let message = match &outcome {
                Ok(n) => format!("✅ {}", n.projname()),
                Err(e) => format!("❌ {e:?}"),
            };
            self.outcome.replace(outcome);
            ui.label(message);
        })
        .response
    }
}
