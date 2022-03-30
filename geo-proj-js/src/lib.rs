#![warn(clippy::unwrap_used)]

use geo::algorithm::map_coords::MapCoordsInplace;
use wasm_bindgen::JsCast;

pub fn transform(geometry: &mut geo::Geometry<f64>, source_crs: &str, target_crs: &str) {
    let proj4 = web_sys::window()
        .unwrap()
        .get("proj4")
        .unwrap()
        .dyn_into::<js_sys::Function>()
        .unwrap();
    let projector = proj4
        .call2(
            &wasm_bindgen::JsValue::UNDEFINED,
            &source_crs.into(),
            &target_crs.into(),
        )
        .unwrap();
    let array = js_sys::Array::new_with_length(2);
    let forward = js_sys::Reflect::get(&projector, &"forward".into())
        .unwrap()
        .dyn_into::<js_sys::Function>()
        .unwrap();
    geometry.map_coords_inplace(|(x, y)| {
        array.set(0, wasm_bindgen::JsValue::from_f64(*x));
        array.set(1, wasm_bindgen::JsValue::from_f64(*y));
        let result = forward
            .call1(&wasm_bindgen::JsValue::UNDEFINED, &array)
            .unwrap()
            .dyn_into::<js_sys::Array>()
            .unwrap();
        (
            result.get(0).as_f64().unwrap(),
            result.get(1).as_f64().unwrap(),
        )
    });
}
