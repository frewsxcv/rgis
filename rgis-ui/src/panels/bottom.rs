use bevy::prelude::*;
use bevy_egui::egui;
use rgis_ui_events::OpenChangeCrsWindow;

pub struct Bottom<'a, 'w> {
    pub egui_ctx: &'a egui::Context,
    pub mouse_pos: &'a rgis_mouse::MousePos,
    pub target_crs: &'a rgis_crs::TargetCrs,
    pub open_change_crs_window_event_writer: &'a mut MessageWriter<'w, OpenChangeCrsWindow>,
    pub bottom_panel_height: &'a mut rgis_units::BottomPanelHeight,
}

fn coordinate_precision(epsg_code: Option<u16>) -> usize {
    match epsg_code {
        Some(code) if (4000..5000).contains(&code) => 6,
        _ => 2,
    }
}

impl Bottom<'_, '_> {
    pub fn render(&mut self) {
        let inner_response = egui::TopBottomPanel::bottom("bottom").show(self.egui_ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.render_crs(ui);
                    ui.separator();
                    self.render_mouse_position(ui);
                });
            });
        });
        self.bottom_panel_height.0 = inner_response.response.rect.height();
    }

    fn render_crs(&mut self, ui: &mut egui::Ui) {
        let edit_btn = ui.button("Edit");
        crate::widget_registry::register("Edit CRS", edit_btn.rect);
        if edit_btn.clicked() {
            self.open_change_crs_window_event_writer.write_default();
        }

        match self.target_crs.0.epsg_code {
            Some(code) => ui.label(format!("CRS: EPSG:{code}")),
            None => ui.label("CRS: Custom PROJ"),
        };
    }

    fn render_mouse_position(&mut self, ui: &mut egui::Ui) {
        let x = self.mouse_pos.0.x.0;
        let y = self.mouse_pos.0.y.0;
        let prec = coordinate_precision(self.target_crs.0.epsg_code);
        ui.label(format!("X: {x:.prec$}  Y: {y:.prec$}"));
    }
}
