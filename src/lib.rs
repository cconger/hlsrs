pub mod parser;
mod utils;

use std::str;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run(input: &[u8], _vsid: &str) -> Result<JsValue, JsValue> {
    let s = str::from_utf8(input).unwrap();
    Ok(JsValue::from_str(&s))
}
