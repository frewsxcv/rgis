use bevy::ecs::system::SystemParam;
use bevy_egui::egui;
use std::marker;

#[derive(SystemParam)]
pub(crate) struct WelcomeWindow<'w, 's> {
    _phantom: marker::PhantomData<(&'w (), &'s ())>,
}

impl<'w, 's> egui::Widget for WelcomeWindow<'w, 's> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical_centered_justified(|ui| {
            ui.label("Welcome to rgis!");
        })
        .response
    }
}

impl crate::window::Window for WelcomeWindow<'_, '_> {
    type Item<'w, 's> = WelcomeWindow<'w, 's>;

    fn title(&self) -> &str {
        "Welcome"
    }

    fn default_width(&self) -> f32 {
        300.0
    }

    fn default_anchor(&self) -> (egui::Align2, [f32; 2]) {
        (egui::Align2::CENTER_CENTER, [0., 0.])
    }
}
