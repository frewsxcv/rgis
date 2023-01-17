use bevy_egui::egui;

pub struct CrsInput<'a> {
    pub outcome: &'a mut Option<Outcome>,
    text_field_value: &'a mut String,
}

pub type Outcome = Result<String, LookupCrsError>;

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
            let edit_field = ui.text_edit_singleline(self.text_field_value);
            let outcome = if edit_field.changed()
                || (!self.text_field_value.is_empty() && self.outcome.is_none())
            {
                lookup_crs(self.text_field_value)
            } else {
                match self.outcome.take() {
                    Some(n) => n,
                    None => return,
                }
            };
            let message = match &outcome {
                Ok(n) => format!("✅ {n}"),
                Err(e) => format!("❌ {e:?}"),
            };
            self.outcome.replace(outcome);
            ui.label(message);
        })
        .response
    }
}

#[cfg(target_arch = "wasm32")]
fn lookup_crs(query: &str) -> Result<String, geo_proj_js::Error> {
    geo_proj_js::lookup_crs(query)
}
#[cfg(target_arch = "wasm32")]
type LookupCrsError = geo_proj_js::Error;

#[cfg(not(target_arch = "wasm32"))]
fn lookup_crs(query: &str) -> Result<String, geo::algorithm::proj::ProjCreateError> {
    geo::algorithm::proj::Proj::new(query).map(|p| {
        p.proj_info()
            .description
            .unwrap_or_else(|| query.to_string())
    })
}
#[cfg(not(target_arch = "wasm32"))]
type LookupCrsError = geo::algorithm::proj::ProjCreateError;
