use bevy_egui::egui;

pub struct CrsInput<'a> {
    pub result: Result<String, LookupCrsError>,
    text_field_value: &'a mut String,
}

impl<'a> CrsInput<'a> {
    pub fn new(text_field_value: &'a mut String) -> Self {
        let result = lookup_crs(text_field_value);
        CrsInput {
            result,
            text_field_value,
        }
    }
}

impl<'a> egui::Widget for CrsInput<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            let edit_field = ui.text_edit_singleline(self.text_field_value);
            if edit_field.changed() {}
            // TODO: DONT CALL THIS ON EVERY LOOP
            let message = match &self.result {
                Ok(n) => format!("✅ {n}"),
                Err(e) => format!("❌ {e:?}"),
            };
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
