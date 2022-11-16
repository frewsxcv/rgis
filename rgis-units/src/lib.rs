#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

// From top-left
#[derive(Copy, Clone, Debug)]
pub struct ScreenCoord {
    pub x: f64,
    pub y: f64,
}

impl ScreenCoord {
    fn to_dvec2(self) -> bevy::math::DVec2 {
        bevy::math::DVec2::new(self.x, self.y)
    }

    pub fn to_projected_geo_coord(
        self,
        transform: &bevy::transform::components::Transform,
        window: &bevy::prelude::Window,
    ) -> geo_projected::Projected<geo::Coord> {
        let size = bevy::math::DVec2::new(f64::from(window.width()), f64::from(window.height()));

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = self.to_dvec2() - size / 2.0;

        // apply the camera transform
        let pos_wld = transform.compute_matrix().as_dmat4() * p.extend(0.0).extend(1.0);

        geo_projected::Projected(geo::Coord {
            x: pos_wld.x,
            y: pos_wld.y,
        })
    }
}

pub struct MapArea<'a> {
    pub window: &'a bevy::window::Window,
    /// Size of UI components (in pixels)
    pub left_offset_px: f32,
    pub top_offset_px: f32,
    pub right_offset_px: f32,
    pub bottom_offset_px: f32,
}

impl<'a> MapArea<'a> {
    fn top_left_screen_coord(&self) -> ScreenCoord {
        ScreenCoord {
            x: f64::from(self.left_offset_px),
            y: f64::from(self.top_offset_px),
        }
    }

    fn top_left_projected_geo_coord(
        &self,
        transform: &bevy::transform::components::Transform,
        window: &bevy::prelude::Window,
    ) -> geo_projected::Projected<geo::Coord> {
        self.top_left_screen_coord()
            .to_projected_geo_coord(transform, window)
    }

    fn bottom_right_screen_coord(&self) -> ScreenCoord {
        ScreenCoord {
            x: f64::from(self.window.width() - self.right_offset_px),
            y: f64::from(self.window.height() - self.bottom_offset_px),
        }
    }

    fn bottom_right_projected_geo_coord(
        &self,
        transform: &bevy::transform::components::Transform,
        window: &bevy::prelude::Window,
    ) -> geo_projected::Projected<geo::Coord> {
        self.bottom_right_screen_coord()
            .to_projected_geo_coord(transform, window)
    }

    pub fn projected_geo_rect(
        &self,
        transform: &bevy::transform::components::Transform,
        window: &bevy::prelude::Window,
    ) -> geo_projected::Projected<geo::Rect> {
        geo_projected::Projected(geo::Rect::new(
            self.top_left_projected_geo_coord(transform, window).0,
            self.bottom_right_projected_geo_coord(transform, window).0,
        ))
    }

    fn width(&self) -> ScreenLength {
        ScreenLength(self.window.width() - self.left_offset_px - self.right_offset_px)
    }

    fn height(&self) -> ScreenLength {
        ScreenLength(self.window.height() - self.top_offset_px - self.bottom_offset_px)
    }

    pub fn size(&self) -> ScreenSize {
        ScreenSize::from_width_height(self.width(), self.height())
    }
}

pub struct ScreenLength(pub f32);

pub struct ScreenSize {
    pub width: f32,
    pub height: f32,
}

impl ScreenSize {
    fn from_width_height(width: ScreenLength, height: ScreenLength) -> Self {
        ScreenSize {
            width: width.0,
            height: height.0,
        }
    }

    pub fn to_bevy_size(&self) -> bevy::ui::Size {
        bevy::ui::Size::new(
            bevy::ui::Val::Px(self.width),
            bevy::ui::Val::Px(self.height),
        )
    }
}
