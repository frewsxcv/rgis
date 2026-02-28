use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use std::marker;

pub trait Window: egui::Widget + SystemParam + Send + Sync {
    type Item<'world, 'state>: Window<State = Self::State>;

    const INITIALLY_OPEN: bool = false;

    fn title(&self) -> &str;
    fn default_width(&self) -> f32;
    fn default_anchor(&self) -> (egui::Align2, [f32; 2]) {
        (egui::Align2::LEFT_TOP, [0., 0.])
    }

    fn setup(app: &mut App)
    where
        Self: 'static,
    {
        app.insert_resource(IsWindowOpen::<Self>(
            Self::INITIALLY_OPEN,
            marker::PhantomData,
        ));
    }
}

pub fn render_window_system<W: Window + 'static>(
    window: <W as Window>::Item<'_, '_>,
    mut bevy_egui_ctx: EguiContexts,
    mut is_window_open: ResMut<IsWindowOpen<W>>,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;

    let (anchor_align, anchor_offset) = window.default_anchor();

    egui::Window::new(window.title())
        .default_width(window.default_width())
        .open(&mut is_window_open.0)
        .resizable(false)
        .anchor(anchor_align, anchor_offset)
        .show(bevy_egui_ctx_mut, |ui| {
            ui.add(window);
        });

    Ok(())
}

pub fn run_if_is_window_open<W: Window + 'static>(is_window_open: Res<IsWindowOpen<W>>) -> bool {
    is_window_open.0
}

pub struct IsWindowOpen<W: Window>(pub bool, marker::PhantomData<W>);

impl<W: Window + 'static> Resource for IsWindowOpen<W> {}

#[cfg(test)]
mod tests {
    use super::{render_window_system, Window};
    use bevy::{
        ecs::system::{assert_is_system, SystemParam},
        prelude::*,
    };
    use bevy_egui::egui::{self, Widget};

    #[derive(SystemParam)]
    struct FakeWindow;

    impl Window for FakeWindow {
        type Item<'world, 'state> = Self;

        fn title(&self) -> &str {
            "Fake Window"
        }

        fn default_width(&self) -> f32 {
            100.0
        }
    }

    impl Widget for FakeWindow {
        fn ui(self, ui: &mut egui::Ui) -> egui::Response {
            ui.label("This is a fake window")
        }
    }

    #[test]
    fn assert_system() {
        assert_is_system(render_window_system::<FakeWindow>.map(|_: Result| {}));
    }
}
