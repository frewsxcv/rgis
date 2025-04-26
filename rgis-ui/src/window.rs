use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use std::marker;

pub trait Window: egui::Widget + SystemParam + Send + Sync {
    type Item<'world, 'state>: Window<State = Self::State>;

    fn title(&self) -> &str;
    fn default_width(&self) -> f32;
    fn default_anchor(&self) -> (egui::Align2, [f32; 2]) {
        (egui::Align2::LEFT_TOP, [0., 0.])
    }
}

pub fn render_window_system<W: Window + 'static>(
    window: <W as Window>::Item<'_, '_>,
    mut egui_ctx: EguiContexts,
    mut is_window_open: ResMut<IsWindowOpen<W>>,
) {
    let (anchor_align, anchor_offset) = window.default_anchor();

    egui::Window::new(window.title())
        .default_width(window.default_width())
        .open(&mut is_window_open.0)
        .resizable(false)
        .anchor(anchor_align, anchor_offset)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.add(window);
        });
}

pub fn run_if_is_window_open<W: Window + 'static>(is_window_open: Res<IsWindowOpen<W>>) -> bool {
    is_window_open.0
}

#[derive(Resource)]
pub struct IsWindowOpen<W: Window + Send + Sync>(pub bool, marker::PhantomData<W>);

impl<W: Window + Send + Sync> IsWindowOpen<W> {
    pub fn closed() -> Self {
        Self(false, marker::PhantomData)
    }

    pub fn open() -> Self {
        Self(true, marker::PhantomData)
    }
}
