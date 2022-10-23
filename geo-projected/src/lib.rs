#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

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

impl<GeometrySelf, GeometryOther> geo::Contains<Projected<GeometryOther>> for Projected<&GeometrySelf>
where
    GeometrySelf: geo::Contains<GeometryOther>,
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

impl<GeometrySelf, GeometryOther> geo::Contains<Unprojected<GeometryOther>>
    for Unprojected<&GeometrySelf>
where
    GeometrySelf: geo::Contains<GeometryOther>,
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

macro_rules! feature_impl_ref {
    ($outer_ty:ident, $inner_ty:ty) => {
        impl $outer_ty<$inner_ty> {
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

feature_impl_ref!(Projected, geo_features::Feature);
feature_impl_ref!(Projected, &geo_features::Feature);
feature_impl_ref!(Unprojected, geo_features::Feature);
feature_impl_ref!(Unprojected, &geo_features::Feature);

macro_rules! feature_collection_impl_ref_mut {
    ($outer_ty:ident, $inner_ty:ty) => {
        impl $outer_ty<$inner_ty> {
            pub fn features_iter_mut(
                &mut self,
            ) -> impl Iterator<Item = Projected<&mut geo_features::Feature>> {
                self.0.features.iter_mut().map(Projected)
            }
        }
    };
}

feature_collection_impl_ref_mut!(Projected, geo_features::FeatureCollection);
feature_collection_impl_ref_mut!(Projected, &mut geo_features::FeatureCollection);
feature_collection_impl_ref_mut!(Unprojected, geo_features::FeatureCollection);
feature_collection_impl_ref_mut!(Unprojected, &mut geo_features::FeatureCollection);

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
