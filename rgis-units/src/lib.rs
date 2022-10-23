#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use geo::Contains;

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

#[derive(Debug)]
pub struct Projected<G>(pub G);

impl<G> Projected<G> {
    pub fn new(g: G) -> Projected<G> {
        Projected(g)
    }

    pub fn as_ref(&self) -> Projected<&G> {
        Projected(&self.0)
    }

    pub fn as_raw(&self) -> &G {
        &self.0
    }

    pub fn into_unprojected(self) -> Unprojected<G> {
        Unprojected(self.0)
    }
}

impl<GeometrySelf, GeometryOther> Contains<Projected<GeometryOther>> for Projected<&GeometrySelf>
where
    GeometrySelf: Contains<GeometryOther>,
{
    fn contains(&self, other: &Projected<GeometryOther>) -> bool {
        self.0.contains(&other.0)
    }
}

impl<G: Copy> Copy for Projected<G> {}
impl<G: Clone> Clone for Projected<G> {
    fn clone(&self) -> Self {
        Projected(self.0.clone())
    }
}

#[derive(Debug)]
pub struct Unprojected<G>(pub G);

impl<G> Unprojected<G> {
    pub fn new(g: G) -> Unprojected<G> {
        Unprojected(g)
    }

    pub fn as_ref(&self) -> Unprojected<&G> {
        Unprojected(&self.0)
    }

    pub fn as_raw(&self) -> &G {
        &self.0
    }

    pub fn into_projected(self) -> Projected<G> {
        Projected(self.0)
    }
}

impl<GeometrySelf, GeometryOther> Contains<Unprojected<GeometryOther>>
    for Unprojected<&GeometrySelf>
where
    GeometrySelf: Contains<GeometryOther>,
{
    fn contains(&self, other: &Unprojected<GeometryOther>) -> bool {
        self.0.contains(&other.0)
    }
}

impl<G: Copy> Copy for Unprojected<G> {}
impl<G: Clone> Clone for Unprojected<G> {
    fn clone(&self) -> Self {
        Unprojected(self.0.clone())
    }
}

macro_rules! feature_impl {
    ($outer_ty:ident, $general_inner_ty:ty, $specific_inner_ty:ty) => {
        impl $outer_ty<$specific_inner_ty> {
            pub fn id(&self) -> geo_features::FeatureId {
                self.0.id
            }

            pub fn properties(&self) -> &geo_features::Properties {
                &self.0.properties
            }

            pub fn geometry(&self) -> Option<$outer_ty<&geo::Geometry>> {
                self.0.geometry.as_ref().map($outer_ty)
            }
        }
    };
}

feature_impl!(Unprojected, geo_features::Feature, geo_features::Feature);
feature_impl!(Unprojected, geo_features::Feature, &geo_features::Feature);
feature_impl!(Projected, geo_features::Feature, geo_features::Feature);
feature_impl!(Projected, geo_features::Feature, &geo_features::Feature);

impl Projected<geo_features::FeatureCollection> {
    pub fn features_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = Projected<&mut geo_features::Feature>> {
        self.0.features.iter_mut().map(Projected)
    }
}

impl Unprojected<geo_features::FeatureCollection> {
    pub fn features_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = Unprojected<&mut geo_features::Feature>> {
        self.0.features.iter_mut().map(Unprojected)
    }
}

macro_rules! feature_collection_impl {
    ($outer_ty:ident, $general_inner_ty:ty, $specific_inner_ty:ty) => {
        impl $outer_ty<$specific_inner_ty> {
            pub fn from_geometry(
                geometry: geo::Geometry,
            ) -> Result<$outer_ty<$general_inner_ty>, geo_features::BoundingRectError> {
                <$general_inner_ty>::from_geometry(geometry).map($outer_ty)
            }

            pub fn features_iter(&self) -> impl Iterator<Item = $outer_ty<&geo_features::Feature>> {
                self.0.features.iter().map($outer_ty)
            }

            pub fn bounding_rect(
                &self,
            ) -> Result<$outer_ty<geo::Rect>, geo_features::BoundingRectError> {
                self.0.bounding_rect().map($outer_ty)
            }

            pub fn to_geometry_collection(&self) -> $outer_ty<geo::GeometryCollection> {
                $outer_ty(self.0.to_geometry_collection())
            }
        }
    };
}

feature_collection_impl!(
    Unprojected,
    geo_features::FeatureCollection,
    geo_features::FeatureCollection
);
feature_collection_impl!(
    Unprojected,
    geo_features::FeatureCollection,
    &geo_features::FeatureCollection
);
feature_collection_impl!(
    Projected,
    geo_features::FeatureCollection,
    geo_features::FeatureCollection
);
feature_collection_impl!(
    Projected,
    geo_features::FeatureCollection,
    &geo_features::FeatureCollection
);

pub struct ScreenLength(pub f32);

pub struct ScreenSize(pub bevy::ui::Size<f32>);

impl ScreenSize {
    fn from_width_height(width: ScreenLength, height: ScreenLength) -> Self {
        ScreenSize(bevy::ui::Size::new(width.0, height.0))
    }
}
