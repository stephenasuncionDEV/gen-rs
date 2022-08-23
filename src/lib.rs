#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]

pub use wasm_bindgen::prelude::*;
pub use log::{Level, info, debug};
pub use futures::future::try_join_all;
pub use wasm_bindgen_futures::JsFuture;
pub use chrono::Utc;
use utils::layer_constructor::construct_layers;
mod structs;
mod utils;

#[wasm_bindgen]
pub async fn createGenRS(layers: JsValue, image_type: String) -> JsValue {
    let input_layers: Vec<structs::InputLayer> = layers.into_serde().unwrap();

    let constructed_layers: Vec<structs::Layer> = construct_layers(
        input_layers, 
        image_type
    ).await.unwrap();

    JsValue::from_serde(&constructed_layers).unwrap()
}


#[wasm_bindgen(start)]
pub async fn main() {
    console_log::init_with_level(Level::Debug).ok();
}