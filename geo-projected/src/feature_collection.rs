use crate::{Projected, Unprojected};

macro_rules! feature_collection_impl_owned {
    ($outer_ty:ident, $inner_ty:ty) => {
        impl $outer_ty<$inner_ty> {
            pub fn into_features_iter(
                self,
            ) -> impl Iterator<Item = $outer_ty<geo_features::Feature>> {
                self.0.features.into_iter().map($outer_ty)
            }
        }
    };
}

feature_collection_impl_owned!(Projected, geo_features::FeatureCollection);
feature_collection_impl_owned!(Unprojected, geo_features::FeatureCollection);

macro_rules! feature_collection_impl_ref_mut {
    ($outer_ty:ident, $inner_ty:ty) => {
        impl $outer_ty<$inner_ty> {
            pub fn features_iter_mut(
                &mut self,
            ) -> impl Iterator<Item = $outer_ty<&mut geo_features::Feature>> {
                self.0.features.iter_mut().map($outer_ty)
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

            pub fn to_geometry_collection_geometry(&self) -> $outer_ty<geo::Geometry> {
                $outer_ty(geo::Geometry::GeometryCollection(
                    self.0.to_geometry_collection(),
                ))
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
