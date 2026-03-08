use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::egui;
use bevy_egui::EguiContexts;
use std::marker;

pub trait Window: egui::Widget + SystemParam + Send + Sync {
    type Item<'world, 'state>: Window<State = Self::State>;

    const INITIALLY_OPEN: bool = false;

    fn title(&self) -> &str;
    fn default_width(&self) -> f32;

    fn setup(app: &mut App)
    where
        Self: 'static,
    {
        if Self::INITIALLY_OPEN {
            app.insert_state(WindowVisibility::<Self>::Open);
        } else {
            app.insert_state(WindowVisibility::<Self>::Closed);
        }
    }
}

pub fn render_window_system<W: Window + 'static>(
    window: <W as Window>::Item<'_, '_>,
    mut bevy_egui_ctx: EguiContexts,
    current_state: Res<State<WindowVisibility<W>>>,
    mut next_state: ResMut<NextState<WindowVisibility<W>>>,
    side_panel_width: Res<rgis_units::SidePanelWidth>,
    top_panel_height: Res<rgis_units::TopPanelHeight>,
) -> Result {
    let bevy_egui_ctx_mut = bevy_egui_ctx.ctx_mut()?;
    let default_pos = egui::pos2(side_panel_width.0 + 4.0, top_panel_height.0 + 4.0);

    let mut is_open = *current_state.get() == WindowVisibility::<W>::Open;

    egui::Window::new(window.title())
        .default_width(window.default_width())
        .open(&mut is_open)
        .resizable(false)
        .default_pos(default_pos)
        .show(bevy_egui_ctx_mut, |ui| {
            ui.add(window);
        });

    // Sync egui close-button back to state
    if !is_open {
        next_state.set(WindowVisibility::<W>::Closed);
    }

    Ok(())
}

/// Run condition: returns true when the window state is `Open`.
pub fn run_if_is_window_open<W: Window + 'static>(
    current_state: Res<State<WindowVisibility<W>>>,
) -> bool {
    *current_state.get() == WindowVisibility::<W>::Open
}

/// Bevy State enum tracking whether a window is open or closed.
pub enum WindowVisibility<W: Window> {
    Open,
    Closed,
    #[doc(hidden)]
    _Marker(marker::PhantomData<W>, std::convert::Infallible),
}

// Manual trait implementations to avoid requiring W: Trait bounds.

impl<W: Window> Clone for WindowVisibility<W> {
    fn clone(&self) -> Self {
        match self {
            Self::Open => Self::Open,
            Self::Closed => Self::Closed,
            Self::_Marker(_, infallible) => match *infallible {},
        }
    }
}

impl<W: Window> Copy for WindowVisibility<W> {}

impl<W: Window> PartialEq for WindowVisibility<W> {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl<W: Window> Eq for WindowVisibility<W> {}

impl<W: Window> core::hash::Hash for WindowVisibility<W> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl<W: Window> core::fmt::Debug for WindowVisibility<W> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::Closed => write!(f, "Closed"),
            Self::_Marker(_, infallible) => match *infallible {},
        }
    }
}

impl<W: Window + 'static> States for WindowVisibility<W> {}

impl<W: Window + 'static> bevy::state::state::FreelyMutableState for WindowVisibility<W> {}

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
