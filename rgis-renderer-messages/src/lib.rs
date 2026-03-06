use bevy::prelude::*;

#[derive(Event)]
pub struct DespawnMeshesEvent(pub rgis_primitives::LayerId);

#[derive(Message)]
pub struct MeshesSpawnedMessage(pub rgis_primitives::LayerId);

impl From<rgis_primitives::LayerId> for MeshesSpawnedMessage {
    #[inline]
    fn from(layer_id: rgis_primitives::LayerId) -> Self {
        MeshesSpawnedMessage(layer_id)
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MeshesSpawnedMessage>();
    }
}
