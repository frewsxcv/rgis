use crate::{Projected, Unprojected};

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
