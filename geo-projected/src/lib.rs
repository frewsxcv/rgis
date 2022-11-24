#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

mod feature;
mod feature_collection;

#[derive(Debug)]
pub struct Projected<G>(pub G); // TODO: remove `pub`

impl<G> Projected<G> {
    pub fn into_unprojected(self) -> Unprojected<G> {
        Unprojected(self.0)
    }
}

#[derive(Debug)]
pub struct Unprojected<G>(pub G); // TODO: remove `pub`

impl<G: Default> Default for Unprojected<G> {
    fn default() -> Self {
        Unprojected(G::default())
    }
}

impl<G> Unprojected<G> {
    pub fn into_projected(self) -> Projected<G> {
        Projected(self.0)
    }
}

macro_rules! base_impls {
    ($ty:ident) => {
        impl<G> $ty<G> {
            pub fn new(g: G) -> $ty<G> {
                $ty(g)
            }

            pub fn as_ref(&self) -> $ty<&G> {
                $ty(&self.0)
            }

            pub fn as_raw(&self) -> &G {
                &self.0
            }
        }

        impl<G: Clone> $ty<&G> {
            pub fn cloned(&self) -> $ty<G> {
                $ty(self.0.clone())
            }
        }

        impl<G: Copy> Copy for $ty<G> {}

        impl<G: Clone> Clone for $ty<G> {
            fn clone(&self) -> Self {
                $ty(self.0.clone())
            }
        }

        impl<GeometrySelf, GeometryOther> geo::Contains<$ty<GeometryOther>> for $ty<&GeometrySelf>
        where
            GeometrySelf: geo::Contains<GeometryOther>,
        {
            fn contains(&self, other: &$ty<GeometryOther>) -> bool {
                self.0.contains(&other.0)
            }
        }
    };
}

base_impls!(Projected);
base_impls!(Unprojected);
