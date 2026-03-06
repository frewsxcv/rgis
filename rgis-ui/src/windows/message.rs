use bevy_egui::egui;

pub struct Message<'a> {
    pub egui_ctx: &'a mut bevy_egui::egui::Context,
    pub state: &'a mut crate::MessageWindowState,
}

impl Message<'_> {
    pub fn render(&mut self) {
        let Some(ref message) = *self.state else {
            return;
        };

        let mut is_open = true;
        egui::Window::new("Message Window")
            .id(egui::Id::new("Message window"))
            .open(&mut is_open)
            .anchor(egui::Align2::CENTER_CENTER, [0., 0.])
            .show(self.egui_ctx, |ui| {
                ui.label(message);
            });

        if !is_open {
            *self.state = None;
        }
    }
}
