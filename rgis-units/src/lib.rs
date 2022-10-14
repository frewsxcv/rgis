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
    ) -> Projected<geo::Coordinate> {
        let size = bevy::math::DVec2::new(f64::from(window.width()), f64::from(window.height()));

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = self.to_dvec2() - size / 2.0;

        // apply the camera transform
        let pos_wld = transform.compute_matrix().as_dmat4() * p.extend(0.0).extend(1.0);

        Projected(geo::Coordinate {
            x: pos_wld.x,
            y: pos_wld.y,
        })
    }
}

pub struct MapArea<'a> {
    pub window: &'a bevy::window::Window,
    /// Size of UI components (in pixels)
    pub ui_rect: bevy::ui::UiRect<f32>,
}

impl<'a> MapArea<'a> {
    fn top_left_screen_coord(&self) -> ScreenCoord {
        ScreenCoord {
            x: f64::from(self.ui_rect.left),
            y: f64::from(self.ui_rect.top),
        }
    }

    fn top_left_projected_geo_coord(
        &self,
        transform: &bevy::transform::components::Transform,
        window: &bevy::prelude::Window,
    ) -> Projected<geo::Coordinate> {
            self.top_left_screen_coord()
                .to_projected_geo_coord(transform, window)
    }

    fn bottom_right_screen_coord(&self) -> ScreenCoord {
        ScreenCoord {
            x: f64::from(self.window.width() - self.ui_rect.right),
            y: f64::from(self.window.height() - self.ui_rect.bottom),
        }
    }

    fn bottom_right_projected_geo_coord(
        &self,
        transform: &bevy::transform::components::Transform,
        window: &bevy::prelude::Window,
    ) -> Projected<geo::Coordinate> {
            self.bottom_right_screen_coord()
                .to_projected_geo_coord(transform, window)
    }

    pub fn projected_geo_rect(
        &self,
        transform: &bevy::transform::components::Transform,
        window: &bevy::prelude::Window,
    ) -> Projected<geo::Rect> {
        Projected(geo::Rect::new(
            self.top_left_projected_geo_coord(transform, window).0,
            self.bottom_right_projected_geo_coord(transform, window).0,
        ))
    }

    fn width(&self) -> ScreenLength {
        ScreenLength(self.window.width() - self.ui_rect.left - self.ui_rect.right)
    }

    fn height(&self) -> ScreenLength {
        ScreenLength(self.window.height() - self.ui_rect.top - self.ui_rect.bottom)
    }

    pub fn size(&self) -> ScreenSize {
        ScreenSize::from_width_height(self.width(), self.height())
    }
}

pub struct Projected<G>(pub G);

impl<G: Copy> Copy for Projected<G> {}
impl<G: Clone> Clone for Projected<G> {
    fn clone(&self) -> Self {
        Projected(self.0.clone())
    }
}

#[derive(Clone)]
pub struct Unprojected<G: Clone>(pub G);

pub struct ScreenLength(pub f32);

pub struct ScreenSize(pub bevy::ui::Size<f32>);

impl ScreenSize {
    fn from_width_height(width: ScreenLength, height: ScreenLength) -> Self {
        ScreenSize(bevy::ui::Size::new(width.0, height.0))
    }
}
