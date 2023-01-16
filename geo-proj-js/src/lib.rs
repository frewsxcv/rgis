#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::indexing_slicing,
    clippy::expect_used
)]

use std::{error, fmt};
use wasm_bindgen::JsCast;

#[derive(Debug)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Failed to transform with proj.js")
    }
}

impl error::Error for Error {}

pub fn transform<G: geo::MapCoordsInPlace<f64>>(
    geometry: &mut G,
    source_crs: &str,
    target_crs: &str,
) -> Result<(), Error> {
    let proj4 = web_sys::window()
        .ok_or(Error)?
        .get("proj4")
        .ok_or(Error)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| Error)?;
    let projector = proj4
        .call2(
            &wasm_bindgen::JsValue::UNDEFINED,
            &source_crs.into(),
            &target_crs.into(),
        )
        .map_err(|_| Error)?;
    let array = js_sys::Array::new_with_length(2);
    let forward = js_sys::Reflect::get(&projector, &"forward".into())
        .map_err(|_| Error)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| Error)?;

    geometry.map_coords_in_place(
        |geo::Coord { x, y }| match in_place((x, y), &forward, &array) {
            Ok(n) => n,
            Err(e) => {
                log::error!("Failed to convert coordinate: {:?}", e);
                geo::Coord { x, y }
            }
        },
    );

    Ok(())
}

fn in_place(
    (x, y): (f64, f64),
    forward: &js_sys::Function,
    array: &js_sys::Array,
) -> Result<geo::Coord, Error> {
    array.set(0, wasm_bindgen::JsValue::from_f64(x));
    array.set(1, wasm_bindgen::JsValue::from_f64(y));
    let result = forward
        .call1(&wasm_bindgen::JsValue::UNDEFINED, array)
        .map_err(|_| Error)?
        .dyn_into::<js_sys::Array>()
        .map_err(|_| Error)?;
    Ok(geo::Coord {
        x: result.get(0).as_f64().ok_or(Error)?,
        y: result.get(1).as_f64().ok_or(Error)?,
    })
}

pub fn lookup_crs(query: &str) -> Result<String, Error> {
    if query.is_empty() {
        return Err(Error);
    }

    let proj4 = web_sys::window()
        .ok_or(Error)?
        .get("proj4")
        .ok_or(Error)?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| Error)?;
    let projector = proj4
        .call1(&wasm_bindgen::JsValue::UNDEFINED, &query.into())
        .map_err(|_| Error)?;
    let title = js_sys::Reflect::get(&projector, &"title".into()).map_err(|_| Error)?;

    match title.dyn_into::<js_sys::JsString>() {
        Ok(n) => Ok(format!("{n} ({query})")),
        Err(_) => Ok(query.into()),
    }
}
