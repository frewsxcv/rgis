#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use std::{num, sync};

// The starting value is `1` so we can utilize `NonZeroU16`.
static NEXT_ID: sync::atomic::AtomicU16 = sync::atomic::AtomicU16::new(1);

#[derive(
    Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash, bevy::ecs::component::Component,
)]
pub struct LayerId(num::NonZeroU16);

impl Default for LayerId {
    fn default() -> Self {
        Self::new()
    }
}

impl LayerId {
    pub fn new() -> Self {
        LayerId(new_id())
    }
}

fn new_id() -> num::NonZeroU16 {
    // Unsafety: The starting ID is 1 and we always increment.
    unsafe { num::NonZeroU16::new_unchecked(NEXT_ID.fetch_add(1, sync::atomic::Ordering::SeqCst)) }
}
