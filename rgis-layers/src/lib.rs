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

#[derive(Clone, Debug, Resource)]
pub struct Layers {
    // Ordered from bottom to top
    data: Vec<Layer>,
    // ID of the currently selected Layer
    pub selected_layer_id: Option<rgis_layer_id::LayerId>,
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
        coord: geo_projected::Projected<geo::Coord>,
    ) -> impl Iterator<Item = &Layer> {
        self.iter_top_to_bottom()
            .filter(move |layer| match layer.projected_feature_collection {
                Some(ref projected) => projected.as_ref().contains(&coord),
                None => false,
            })
    }

    fn feature_collections_iter(&self) -> impl Iterator<Item = FeatureCollectionsIterItem> {
        self.iter_top_to_bottom().flat_map(|layer| {
            layer
                .projected_feature_collection
                .as_ref()
                .map(|projected| FeatureCollectionsIterItem {
                    layer_id: layer.id,
                    unprojected: &layer.unprojected_feature_collection,
                    projected,
                })
        })
    }

    fn features_iter(&self) -> impl Iterator<Item = FeaturesIterItem> {
        self.feature_collections_iter().flat_map(
            |FeatureCollectionsIterItem {
                 layer_id,
                 projected,
                 unprojected,
             }| {
                unprojected
                    .features_iter()
                    .zip(projected.features_iter())
                    .map(move |(unprojected, projected)| FeaturesIterItem {
                        layer_id,
                        projected,
                        unprojected,
                    })
            },
        )
    }

    pub fn feature_from_click(
        &self,
        coord: geo_projected::Projected<geo::Coord>,
    ) -> Option<(
        rgis_layer_id::LayerId,
        geo_projected::Unprojected<&geo_features::Feature>,
    )> {
        self.features_iter()
            .find(|item| item.projected.contains(&coord))
            .map(|item| (item.layer_id, item.unprojected))
    }

    fn get_index(&self, layer_id: rgis_layer_id::LayerId) -> Option<usize> {
        self.data.iter().position(|entry| entry.id == layer_id)
    }

    #[inline]
    pub fn get(&self, layer_id: rgis_layer_id::LayerId) -> Option<&Layer> {
        let index = self.get_index(layer_id)?;
        self.data.get(index)
    }

    #[inline]
    pub fn get_with_index(&self, layer_id: rgis_layer_id::LayerId) -> Option<(&Layer, LayerIndex)> {
        let index = self.get_index(layer_id)?;
        self.data.get(index).map(|layer| (layer, LayerIndex(index)))
    }

    #[inline]
    pub fn get_mut(&mut self, layer_id: rgis_layer_id::LayerId) -> Option<&mut Layer> {
        let index = self.get_index(layer_id)?;
        self.data.get_mut(index)
    }

    #[inline]
    pub fn remove(&mut self, layer_id: rgis_layer_id::LayerId) {
        if let Some(index) = self.get_index(layer_id) {
            self.data.remove(index);
        }
    }

    #[allow(unused)]
    pub fn selected_layer(&self) -> Option<&Layer> {
        self.selected_layer_id
            .and_then(|layer_id| self.get(layer_id))
    }

    fn next_layer_id(&self) -> rgis_layer_id::LayerId {
        rgis_layer_id::LayerId::new()
    }

    fn add(
        &mut self,
        unprojected: geo_projected::Unprojected<geo_features::FeatureCollection>,
        name: String,
        source_crs: String,
    ) -> Result<rgis_layer_id::LayerId, geo_features::BoundingRectError> {
        let layer_id = self.next_layer_id();
        let geom_type = geo_geom_type::determine(unprojected.as_raw().geometry_iter());
        let layer = Layer {
            unprojected_feature_collection: unprojected,
            projected_feature_collection: None,
            color: colorous_color_to_bevy_color(next_colorous_color()),
            name,
            visible: true,
            id: layer_id,
            crs: source_crs,
            geom_type,
        };
        self.data.push(layer);
        Ok(layer_id)
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

pub type Metadata = serde_json::Map<String, serde_json::Value>;

#[derive(Clone, Debug)]
pub struct Layer {
    pub unprojected_feature_collection: geo_projected::Unprojected<geo_features::FeatureCollection>,
    pub projected_feature_collection:
        Option<geo_projected::Projected<geo_features::FeatureCollection>>,
    pub color: Color,
    pub id: rgis_layer_id::LayerId,
    pub name: String,
    pub visible: bool,
    pub crs: String,
    pub geom_type: geo_geom_type::GeomType,
}

impl Layer {
    #[inline]
    pub fn get_projected_feature_collection_or_log(
        &self,
    ) -> Option<&geo_projected::Projected<geo_features::FeatureCollection>> {
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
    ) -> Option<geo_projected::Projected<&geo_features::Feature>> {
        let feature_collection = self.get_projected_feature_collection_or_log()?;
        feature_collection
            .features_iter()
            .find(|f| f.id() == feature_id)
    }
}

fn colorous_color_to_bevy_color(colorous_color: colorous::Color) -> Color {
    Color::rgb_u8(colorous_color.r, colorous_color.g, colorous_color.b)
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
        app.insert_resource(Layers::new())
            .add_system_set(systems::system_set());
    }
}

struct FeatureCollectionsIterItem<'a> {
    layer_id: rgis_layer_id::LayerId,
    projected: &'a geo_projected::Projected<geo_features::FeatureCollection>,
    unprojected: &'a geo_projected::Unprojected<geo_features::FeatureCollection>,
}

struct FeaturesIterItem<'a> {
    layer_id: rgis_layer_id::LayerId,
    projected: geo_projected::Projected<&'a geo_features::Feature>,
    unprojected: geo_projected::Unprojected<&'a geo_features::Feature>,
}
