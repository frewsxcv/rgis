#[derive(
    Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, bevy::ecs::component::Component,
)]
pub struct LayerId(pub u16);
