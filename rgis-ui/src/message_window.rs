use bevy_egui::egui;

pub(crate) struct MessageWindow<'a> {
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub state: &'a mut crate::MessageWindowState,
}

impl MessageWindow<'_> {
    pub(crate) fn render(&mut self) {
        if !self.state.is_visible && self.state.message.is_some() {
            self.state.message = None;
        }
        if let Some(ref message) = self.state.message {
            egui::Window::new("Message Window")
                .id(egui::Id::new("Message window"))
                .open(&mut self.state.is_visible)
                .anchor(egui::Align2::CENTER_CENTER, [0., 0.])
                .show(self.egui_ctx, |ui| {
                    ui.label(message);
                });
        }
    }
}
