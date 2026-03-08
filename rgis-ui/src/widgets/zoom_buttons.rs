use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn render_zoom_buttons(
    mut bevy_egui_ctx: EguiContexts,
    mut zoom_event_writer: MessageWriter<rgis_events::ZoomCameraMessage>,
    mouse_pos: Res<rgis_mouse::MousePos>,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;

    egui::Window::new("zoom_buttons")
        .title_bar(false)
        .resizable(false)
        .anchor(egui::Align2::RIGHT_TOP, [-8.0, 40.0])
        .show(bevy_egui_ctx_mut, |ui| {
            ui.horizontal(|ui| {
                let zoom_in_btn = ui.button("+");
                crate::widget_registry::register("Zoom In", zoom_in_btn.rect);
                if zoom_in_btn.clicked() {
                    zoom_event_writer
                        .write(rgis_events::ZoomCameraMessage::new(50.0, mouse_pos.0));
                }
                let zoom_out_btn = ui.button("\u{2212}");
                crate::widget_registry::register("Zoom Out", zoom_out_btn.rect);
                if zoom_out_btn.clicked() {
                    zoom_event_writer
                        .write(rgis_events::ZoomCameraMessage::new(-50.0, mouse_pos.0));
                }
            });
        });

    Ok(())
}
