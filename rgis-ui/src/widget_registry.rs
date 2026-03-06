#[cfg(target_arch = "wasm32")]
mod inner {
    use std::cell::RefCell;
    use std::collections::HashMap;

    thread_local! {
        static POSITIONS: RefCell<HashMap<String, [f32; 4]>> = RefCell::new(HashMap::new());
        static CLOSE_REQUESTS: RefCell<Vec<String>> = RefCell::new(Vec::new());
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

    pub fn request_close(title: &str) {
        CLOSE_REQUESTS.with(|r| {
            r.borrow_mut().push(title.to_string());
        });
    }

    pub fn take_close_request(title: &str) -> bool {
        CLOSE_REQUESTS.with(|r| {
            let mut requests = r.borrow_mut();
            if let Some(pos) = requests.iter().position(|t| t == title) {
                requests.remove(pos);
                true
            } else {
                false
            }
        })
    }
}

#[cfg(target_arch = "wasm32")]
pub use inner::*;

#[cfg(not(target_arch = "wasm32"))]
#[inline]
pub fn register(_label: &str, _rect: bevy_egui::egui::Rect) {}

#[cfg(not(target_arch = "wasm32"))]
#[inline]
pub fn take_close_request(_title: &str) -> bool {
    false
}
