#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use bevy::prelude::*;
use geo::contains::Contains;
use std::sync;

mod systems;

#[derive(Copy, Clone, Debug)]
pub struct LayerIndex(pub usize);

#[derive(Debug, Resource)]
pub struct Layers {
    // Ordered from bottom to top
    data: Vec<Layer>,
    // ID of the currently selected Layer
    pub selected_layer_id: Option<rgis_primitives::LayerId>,
}

impl Default for Layers {
    fn default() -> Self {
        Self::new()
    }
}

impl Layers {
    pub fn new() -> Layers {
        Layers {
            data: vec![],
            selected_layer_id: None,
        }
    }

    #[inline]
    pub fn iter_bottom_to_top(&self) -> impl Iterator<Item = &Layer> {
        self.data.iter()
    }

    #[inline]
    pub fn iter_top_to_bottom(&self) -> impl Iterator<Item = &Layer> {
        self.data.iter().rev()
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn containing_coord(
        &self,
        coord: geo_projected::ProjectedCoord,
    ) -> impl Iterator<Item = &Layer> {
        self.iter_top_to_bottom()
            .filter(move |layer| match layer.projected_feature_collection {
                Some(ref projected) => projected.contains(&coord),
                None => false,
            })
    }

    fn feature_collections_iter(&self) -> impl Iterator<Item = FeatureCollectionsIterItem> {
        self.iter_top_to_bottom().flat_map(|layer| {
            layer
                .projected_feature_collection
                .as_ref()
                .map(|projected| FeatureCollectionsIterItem {
                    layer,
                    unprojected: &layer.unprojected_feature_collection,
                    projected,
                })
        })
    }

    fn features_iter(&self) -> impl Iterator<Item = FeaturesIterItem> {
        self.feature_collections_iter().flat_map(
            |FeatureCollectionsIterItem {
                 layer,
                 projected,
                 unprojected,
             }| {
                unprojected
                    .features
                    .iter()
                    .zip(projected.features.iter())
                    .map(move |(unprojected, projected)| FeaturesIterItem {
                        layer,
                        projected,
                        unprojected,
                    })
            },
        )
    }

    pub fn feature_from_click(
        &self,
        coord: geo_projected::ProjectedCoord,
    ) -> Option<(
        &Layer,
        &geo_features::Feature<geo_projected::UnprojectedScalar>,
    )> {
        self.features_iter()
            .find(|item| item.projected.contains(&coord))
            .map(|item| (item.layer, item.unprojected))
    }

    fn get_index(&self, layer_id: rgis_primitives::LayerId) -> Option<usize> {
        self.data.iter().position(|entry| entry.id == layer_id)
    }

    #[inline]
    pub fn get(&self, layer_id: rgis_primitives::LayerId) -> Option<&Layer> {
        let index = self.get_index(layer_id)?;
        self.data.get(index)
    }

    #[inline]
    pub fn get_with_index(
        &self,
        layer_id: rgis_primitives::LayerId,
    ) -> Option<(&Layer, LayerIndex)> {
        let index = self.get_index(layer_id)?;
        self.data.get(index).map(|layer| (layer, LayerIndex(index)))
    }

    #[inline]
    pub fn get_mut(&mut self, layer_id: rgis_primitives::LayerId) -> Option<&mut Layer> {
        let index = self.get_index(layer_id)?;
        self.data.get_mut(index)
    }

    #[inline]
    pub fn remove(&mut self, layer_id: rgis_primitives::LayerId) {
        if let Some(index) = self.get_index(layer_id) {
            self.data.remove(index);
        }
    }

    #[allow(unused)]
    pub fn selected_layer(&self) -> Option<&Layer> {
        self.selected_layer_id
            .and_then(|layer_id| self.get(layer_id))
    }

    fn next_layer_id(&self) -> rgis_primitives::LayerId {
        rgis_primitives::LayerId::new()
    }

    fn add(
        &mut self,
        unprojected: geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
        name: String,
        crs: rgis_primitives::Crs,
    ) -> rgis_primitives::LayerId {
        let layer_id = self.next_layer_id();
        let geom_type = geo_geom_type::determine(unprojected.geometry_iter());
        let layer = Layer {
            unprojected_feature_collection: unprojected,
            projected_feature_collection: None,
            color: if geom_type.has_fill() {
                LayerColor {
                    fill: Some(colorous_color_to_bevy_color(next_colorous_color())),
                    stroke: Color::BLACK,
                }
            } else {
                LayerColor {
                    fill: None,
                    stroke: colorous_color_to_bevy_color(next_colorous_color()),
                }
            },
            name,
            visible: true,
            id: layer_id,
            crs,
            geom_type,
        };
        self.data.push(layer);
        layer_id
    }

    pub fn clear_projected(&mut self) {
        for layer in self.data.iter_mut() {
            layer.projected_feature_collection = None;
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Layer> {
        self.data.iter()
    }
}

#[derive(Clone, Debug)]
pub struct LayerColor {
    pub fill: Option<Color>,
    pub stroke: Color,
}

#[derive(Debug)]
pub struct Layer {
    pub unprojected_feature_collection:
        geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
    pub projected_feature_collection:
        Option<geo_features::FeatureCollection<geo_projected::ProjectedScalar>>,
    pub color: LayerColor,
    pub id: rgis_primitives::LayerId,
    pub name: String,
    pub visible: bool,
    pub crs: rgis_primitives::Crs,
    pub geom_type: geo_geom_type::GeomType,
}

impl Layer {
    pub fn is_active(&self) -> bool {
        self.projected_feature_collection.is_some()
    }

    #[inline]
    pub fn get_projected_feature_collection_or_log(
        &self,
    ) -> Option<&geo_features::FeatureCollection<geo_projected::ProjectedScalar>> {
        match self.projected_feature_collection.as_ref() {
            Some(p) => Some(p),
            None => {
                bevy::log::error!(
                    "Expected layer (id: {:?}) to have a projected feature collection",
                    self.id
                );
                None
            }
        }
    }

    #[inline]
    pub fn get_projected_feature(
        &self,
        feature_id: geo_features::FeatureId,
    ) -> Option<&geo_features::Feature<geo_projected::ProjectedScalar>> {
        let feature_collection = self.get_projected_feature_collection_or_log()?;
        feature_collection
            .features
            .iter()
            .find(|f| f.id == feature_id)
    }
}

fn colorous_color_to_bevy_color(colorous_color: colorous::Color) -> Color {
    Color::srgb_u8(colorous_color.r, colorous_color.g, colorous_color.b)
}

const COLORS: [colorous::Color; 10] = colorous::CATEGORY10;

fn next_colorous_color() -> colorous::Color {
    #[allow(clippy::indexing_slicing)]
    COLORS[next_color_index()]
}

fn next_color_index() -> usize {
    static COUNTER: sync::atomic::AtomicUsize = sync::atomic::AtomicUsize::new(0);
    COUNTER.fetch_add(1, sync::atomic::Ordering::Relaxed) % COLORS.len()
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Layers::new());
        systems::configure(app);
    }
}

struct FeatureCollectionsIterItem<'a> {
    layer: &'a Layer,
    projected: &'a geo_features::FeatureCollection<geo_projected::ProjectedScalar>,
    unprojected: &'a geo_features::FeatureCollection<geo_projected::UnprojectedScalar>,
}

struct FeaturesIterItem<'a> {
    layer: &'a Layer,
    projected: &'a geo_features::Feature<geo_projected::ProjectedScalar>,
    unprojected: &'a geo_features::Feature<geo_projected::UnprojectedScalar>,
}
