use bevy::prelude::*;

#[derive(Message)]
pub struct DespawnMeshesEvent(pub rgis_primitives::LayerId);

#[derive(Message)]
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
        app.add_message::<DespawnMeshesEvent>()
            .add_message::<MeshesSpawnedEvent>();
    }
}
