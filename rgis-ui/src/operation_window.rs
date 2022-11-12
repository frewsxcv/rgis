use bevy_egui::egui;

trait E: std::error::Error {}

pub(crate) struct OperationWindow<'a> {
    pub is_visible: &'a mut bool,
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub operation: Option<Box<dyn rgis_geo_ops::Operation>>,
//     pub text_field_value: &'a mut String,
//     pub change_crs_event_writer:
//         &'a mut bevy::ecs::event::EventWriter<'w, 's, rgis_events::ChangeCrsEvent>,
//     pub rgis_settings: &'a rgis_settings::RgisSettings,
}

impl<'a> OperationWindow<'a> {
    pub(crate) fn render(&mut self) {
	let Some(ref operation) = self.operation else { return };
        egui::Window::new("Operation")
            .open(self.is_visible)
            .anchor(egui::Align2::LEFT_TOP, [5., 5.])
            .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
		operation.ui(ui);
            });
    }
}
