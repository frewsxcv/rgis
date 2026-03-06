use bevy::prelude::*;
use std::collections::HashMap;

/// Maps each `LayerId` to the set of entities that carry it.
///
/// Maintained automatically via `Add` / `Remove` observers so that
/// systems can look up entities for a specific layer in O(1) instead of
/// scanning every render entity.
#[derive(Resource, Default, Debug)]
pub struct RenderEntityIndex {
    map: HashMap<rgis_primitives::LayerId, Vec<Entity>>,
}

impl RenderEntityIndex {
    /// Return the entities associated with the given layer, if any.
    pub fn get(&self, layer_id: rgis_primitives::LayerId) -> &[Entity] {
        self.map
            .get(&layer_id)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    fn insert(&mut self, layer_id: rgis_primitives::LayerId, entity: Entity) {
        self.map.entry(layer_id).or_default().push(entity);
    }

    fn remove(&mut self, layer_id: rgis_primitives::LayerId, entity: Entity) {
        if let Some(entities) = self.map.get_mut(&layer_id) {
            entities.retain(|&e| e != entity);
            if entities.is_empty() {
                self.map.remove(&layer_id);
            }
        }
    }
}

/// Observer triggered when a `LayerId` component is added to an entity.
pub fn on_add_layer_id(
    on: On<Add, rgis_primitives::LayerId>,
    query: Query<&rgis_primitives::LayerId>,
    mut index: ResMut<RenderEntityIndex>,
) {
    let entity = on.entity;
    if let Ok(layer_id) = query.get(entity) {
        index.insert(*layer_id, entity);
    }
}

/// Observer triggered when a `LayerId` component is removed from an entity.
pub fn on_remove_layer_id(
    on: On<Remove, rgis_primitives::LayerId>,
    query: Query<&rgis_primitives::LayerId>,
    mut index: ResMut<RenderEntityIndex>,
) {
    let entity = on.entity;
    if let Ok(layer_id) = query.get(entity) {
        index.remove(*layer_id, entity);
    }
}
