#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use std::sync;

static NEXT_ID: sync::atomic::AtomicU16 = sync::atomic::AtomicU16::new(0);

#[derive(
    Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, bevy::ecs::component::Component,
)]
pub struct LayerId(u16);

impl Default for LayerId {
    fn default() -> Self {
        Self::new()
    }
}

impl LayerId {
    pub fn new() -> Self {
        let id = NEXT_ID.fetch_add(1, sync::atomic::Ordering::SeqCst);
        LayerId(id)
    }
}
