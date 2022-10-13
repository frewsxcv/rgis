#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

pub fn screen_coords_to_geo_coords(
    screen_coords: bevy::prelude::Vec2,
    transform: &bevy::transform::components::Transform,
    window: &bevy::prelude::Window,
) -> ScreenLocation {
    let size = bevy::math::Vec2::new(window.width(), window.height());

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let p = screen_coords - size / 2.0;

    // apply the camera transform
    let pos_wld = transform.compute_matrix() * p.extend(0.0).extend(1.0);

    ScreenLocation {
        x: pos_wld.x.into(),
        y: pos_wld.y.into(),
    }
}

// From top-left
pub struct ScreenLocation {
    x: f64,
    y: f64,
}

fn map_area_projected_rect() -> bevy::ui::UiRect<f64> {
    todo!()
}

fn center_camera_on_screen_coords_rect() {
    todo!()
}

pub struct ScreenLength(f32);

pub struct ScreenSize(bevy::ui::Size<f32>);

impl ScreenSize {
    fn from_width_height(width: ScreenLength, height: ScreenLength) -> Self {
        ScreenSize(bevy::ui::Size::new(width.0, height.0))
    }
}

fn map_area_width(window: &bevy::window::Window, side_panel_width: ScreenLength) -> ScreenLength {
    ScreenLength(window.width() - side_panel_width.0)
}

pub fn map_area_height(
    window: &bevy::window::Window,
    top_panel_height: ScreenLength,
    bottom_panel_height: ScreenLength,
) -> ScreenLength {
    ScreenLength(window.height() - top_panel_height.0 - bottom_panel_height.0)
}

pub fn map_area_size(
    window: &bevy::window::Window,
    side_panel_width: ScreenLength,
    top_panel_height: ScreenLength,
    bottom_panel_height: ScreenLength,
) -> ScreenSize {
    ScreenSize::from_width_height(
        map_area_width(window, side_panel_width),
        map_area_height(window, top_panel_height, bottom_panel_height),
    )
}
