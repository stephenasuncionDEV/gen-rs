pub use wasm_bindgen::prelude::JsValue;
pub use log::info;
use crate::structs;
use super::image_constructor::construct_layer_images;

pub fn construct_layers(layers: &JsValue, image_type: String) -> Vec<structs::Layer> {
    info!("[gen-rs] constructing layers");

    let layers_input_vec: Vec<structs::LayersInput> = layers.into_serde().unwrap();

    layers_input_vec
    .into_iter()
    .map(|layer: structs::LayersInput| structs::Layer {
        name: layer.name,
        images: construct_layer_images(layer.images, &image_type.clone())
    })
    .collect()
}