use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn generate() {
    console::log_1(&"[gen-rb] Generator Started".into());
}