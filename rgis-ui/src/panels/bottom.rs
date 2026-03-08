use bevy_egui::egui;

pub struct Bottom<'a> {
    pub egui_ctx: &'a egui::Context,
    pub mouse_pos: &'a rgis_mouse::MousePos,
    pub target_crs: &'a rgis_crs::TargetCrs,
    pub geodesy_ctx: &'a rgis_crs::GeodesyContext,
    pub wgs84_op_handle: &'a rgis_crs::Wgs84OpHandle,
    pub change_crs_window_visible: &'a mut crate::ChangeCrsWindowVisible,
    pub bottom_panel_height: &'a mut rgis_units::BottomPanelHeight,
}

impl Bottom<'_> {
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
            self.change_crs_window_visible.0 = true;
        }

        match self.target_crs.0.epsg_code {
            Some(code) => ui.label(format!("CRS: EPSG:{code}")),
            None => ui.label("CRS: Custom PROJ"),
        };
    }

    fn render_mouse_position(&mut self, ui: &mut egui::Ui) {
        if let Some((lat, lng)) = self.projected_to_latlng() {
            ui.label(format!("Lat: {lat:.6}  Lng: {lng:.6}"));
        } else {
            let x = self.mouse_pos.0.x.0;
            let y = self.mouse_pos.0.y.0;
            ui.label(format!("X: {x:.2}  Y: {y:.2}"));
        }
    }

    fn projected_to_latlng(&self) -> Option<(f64, f64)> {
        let geodesy_ctx_inner = self.geodesy_ctx.read().ok()?;

        let transformer = geo_geodesy::Transformer::from_geodesy(
            &*geodesy_ctx_inner,
            self.target_crs.0.op_handle,
            self.wgs84_op_handle.0,
        )
        .ok()?;

        let x = self.mouse_pos.0.x.0;
        let y = self.mouse_pos.0.y.0;
        let mut point = geo::Geometry::Point(geo::Point::new(x, y));
        transformer.transform(&mut point).ok()?;

        if let geo::Geometry::Point(p) = point {
            let lng = p.x();
            let lat = p.y();
            if lat.is_finite() && lng.is_finite() {
                return Some((lat, lng));
            }
        }
        None
    }
}
