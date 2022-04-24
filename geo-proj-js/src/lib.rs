#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use geo::algorithm::map_coords::MapCoordsInPlace;
use std::{error, fmt};
use wasm_bindgen::JsCast;

#[derive(Debug)]
struct CouldNotProjectError;

impl fmt::Display for CouldNotProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Failed to transform with proj.js")
    }
}

impl error::Error for CouldNotProjectError {}

pub fn transform(
    geometry: &mut geo::Geometry<f64>,
    source_crs: &str,
    target_crs: &str,
) -> Result<(), Box<dyn error::Error + Send + Sync>> {
    let proj4 = web_sys::window()
        .ok_or(CouldNotProjectError)?
        .get("proj4")
        .ok_or(CouldNotProjectError)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| CouldNotProjectError)?;
    let projector = proj4
        .call2(
            &wasm_bindgen::JsValue::UNDEFINED,
            &source_crs.into(),
            &target_crs.into(),
        )
        .map_err(|_| CouldNotProjectError)?;
    let array = js_sys::Array::new_with_length(2);
    let forward = js_sys::Reflect::get(&projector, &"forward".into())
        .map_err(|_| CouldNotProjectError)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| CouldNotProjectError)?;

    geometry.map_coords_in_place(|(x, y)| match in_place((x, y), &forward, &array) {
        Ok(n) => n,
        Err(e) => {
            log::error!("Failed to convert coordinate: {:?}", e);
            (x, y)
        }
    });

    // geometry.try_map_coords_in_place(|(x, y)| in_place((x, y), &forward, &array))?;

    Ok(())
}

fn in_place(
    (x, y): (f64, f64),
    forward: &js_sys::Function,
    array: &js_sys::Array,
) -> Result<(f64, f64), Box<dyn error::Error + Send + Sync>> {
    array.set(0, wasm_bindgen::JsValue::from_f64(x));
    array.set(1, wasm_bindgen::JsValue::from_f64(y));
    let result = forward
        .call1(&wasm_bindgen::JsValue::UNDEFINED, array)
        .map_err(|_| CouldNotProjectError)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| CouldNotProjectError)?;
    Ok((
        result.get(0).as_f64().ok_or(CouldNotProjectError)?,
        result.get(1).as_f64().ok_or(CouldNotProjectError)?,
    ))
}
