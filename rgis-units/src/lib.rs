#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

pub fn screen_coords_to_geo_coords(
    screen_coords: ScreenLocation,
    transform: &bevy::transform::components::Transform,
    window: &bevy::prelude::Window,
) -> geo::Coordinate {
    let size = bevy::math::DVec2::new(f64::from(window.width()), f64::from(window.height()));

    // the default orthographic projection is in pixels from the center;
    // just undo the translation
    let p = bevy::math::DVec2::new(screen_coords.x, screen_coords.y) - size / 2.0;

    // apply the camera transform
    let pos_wld = transform.compute_matrix().as_dmat4() * p.extend(0.0).extend(1.0);

    geo::Coordinate {
        x: pos_wld.x,
        y: pos_wld.y,
    }
}

// From top-left
pub struct ScreenLocation {
    pub x: f64,
    pub y: f64,
}

fn map_area_projected_rect() -> bevy::ui::UiRect<f64> {
    todo!()
}

fn center_camera_on_screen_coords_rect() {
    todo!()
}

pub struct ScreenLength(pub f32);

pub struct ScreenSize(pub bevy::ui::Size<f32>);

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
