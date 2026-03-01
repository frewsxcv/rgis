#[cfg(target_arch = "wasm32")]
mod inner {
    use std::cell::RefCell;
    use std::collections::HashMap;

    thread_local! {
        static POSITIONS: RefCell<HashMap<String, [f32; 4]>> = RefCell::new(HashMap::new());
    }

    pub fn register(label: &str, rect: bevy_egui::egui::Rect) {
        POSITIONS.with(|p| {
            p.borrow_mut().insert(
                label.to_string(),
                [rect.min.x, rect.min.y, rect.max.x, rect.max.y],
            );
        });
    }

    pub fn get(label: &str) -> Option<[f32; 4]> {
        POSITIONS.with(|p| p.borrow().get(label).copied())
    }

    pub fn get_all() -> HashMap<String, [f32; 4]> {
        POSITIONS.with(|p| p.borrow().clone())
    }
}

#[cfg(target_arch = "wasm32")]
pub use inner::*;

#[cfg(not(target_arch = "wasm32"))]
#[inline]
pub fn register(_label: &str, _rect: bevy_egui::egui::Rect) {}
