extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;
// import nanoid module
extern crate nanoid;

// to be able use JsValue without namespace
use wasm_bindgen::prelude::*;

// the function itself
#[wasm_bindgen]
pub fn simpleNanoid() -> js_sys::JsString {
    js_sys::JsString::from(nanoid::simple())
}
