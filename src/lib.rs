use wasm_bindgen::prelude::*;
use log::Level;
mod structs;
mod utils;

// #[wasm_bindgen]
// pub fn generate(layers: &JsValue) {
//     info!("[gen-rs] Generator Started");
//     info!("[gen-rs] Generator Finished");
// }

#[wasm_bindgen]
pub struct GenRS {
    data: Vec<structs::Layer>,
}

#[wasm_bindgen]
impl GenRS {
    #[wasm_bindgen(constructor)]
    pub fn new(layers: &JsValue, image_type: String) -> JsValue {
        let layers: Vec<structs::Layer> = utils::layer_constructor::construct_layers(layers, image_type);
        JsValue::from_serde(&layers).unwrap()
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(Level::Debug).ok();
}