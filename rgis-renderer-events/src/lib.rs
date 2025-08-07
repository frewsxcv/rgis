use bevy::prelude::*;

#[derive(Event)]
pub struct DespawnMeshesEvent(pub rgis_primitives::LayerId);

#[derive(Event)]
pub struct MeshesSpawnedEvent(pub rgis_primitives::LayerId);

impl From<rgis_primitives::LayerId> for MeshesSpawnedEvent {
    #[inline]
    fn from(layer_id: rgis_primitives::LayerId) -> Self {
        MeshesSpawnedEvent(layer_id)
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnMeshesEvent>()
            .add_event::<MeshesSpawnedEvent>();
    }
}
