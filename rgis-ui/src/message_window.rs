use bevy_egui::egui;

pub(crate) struct MessageWindow<'a> {
    pub bevy_egui_ctx: &'a mut bevy_egui::EguiContext,
    pub state: &'a mut crate::MessageWindowState,
}

impl<'a> MessageWindow<'a> {
    pub(crate) fn render(&mut self) {
        if !self.state.is_visible && self.state.message.is_some() {
            self.state.message = None;
        }
        if let Some(ref message) = self.state.message {
            egui::Window::new("Message Window")
                .id(egui::Id::new("Message window"))
                .open(&mut self.state.is_visible)
                .anchor(egui::Align2::CENTER_CENTER, [0., 0.])
                .show(self.bevy_egui_ctx.ctx_mut(), |ui| {
                    ui.label(message);
                });
        }
    }
}
