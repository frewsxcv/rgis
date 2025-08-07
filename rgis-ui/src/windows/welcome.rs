use bevy::prelude::*;
use bevy_egui::{egui::{self, Layout}, EguiContexts};
use rgis_file_loader_events::LoadFileEvent;
use rgis_primitives::Crs;

use crate::WelcomeWindowOpen;

pub fn render_welcome_window(
    mut load_file_event_writer: EventWriter<LoadFileEvent>,
    geodesy_ctx: Res<rgis_geodesy::GeodesyContext>,
    mut egui_contexts: EguiContexts,
    mut is_window_open: ResMut<WelcomeWindowOpen>,
) {
    if !is_window_open.0 {
        return;
    }

    let egui_ctx = egui_contexts.ctx_mut();

    egui::Window::new("Welcome")
        .default_width(300.0)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0., 0.])
        .show(egui_ctx.unwrap(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Welcome to rgis!");
                ui.separator();
                ui.label("rgis is a free and open source desktop GIS application for viewing and analyzing geospatial data.");
                ui.hyperlink_to("Source code", "https://github.com/d-bucur/rgis");
                ui.separator();
                ui.heading("Start a new session");
                ui.add_space(10.0);
                if ui.button("Empty map").clicked() {
                    is_window_open.0 = false;
                };
                ui.add_space(10.0);
                ui.with_layout(Layout::top_down_justified(egui::Align::Center), |ui| {
                    if ui.button("Load example GeoJSON layers").clicked() {
                        let mut geodesy_ctx = geodesy_ctx.0.write().unwrap();

                        load_file_event_writer.write(LoadFileEvent::FromNetwork {
                            name: "Earthquakes".to_string(),
                            url: "https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/4.5_week.geojson".to_string(),
                            source_crs: Crs {
                                epsg_code: 4326,
                                op_handle: rgis_geodesy::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx, 4326).unwrap(),
                            },
                        });
                        load_file_event_writer.write(LoadFileEvent::FromNetwork {
                            name: "Populated Places".to_string(),
                            url: "https://d2ad6b4ur77vp9.cloudfront.net/naturalearth-3.3.0/ne_50m_populated_places.geojson".to_string(),
                            source_crs: Crs {
                                epsg_code: 4326,
                                op_handle: rgis_geodesy::epsg_code_to_geodesy_op_handle(&mut *geodesy_ctx, 4326).unwrap(),
                            },
                        });
                        is_window_open.0 = false;
                    };
                });
            });
        });
}
