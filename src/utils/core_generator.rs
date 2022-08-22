use crate::{structs, info, debug};
use wasm_bindgen::prelude::*;
use photon_rs::{multiple, base64_to_image};
use rand::prelude::*;
use rand::distributions::WeightedIndex;

pub fn randomized_image(layer: structs::Layer) -> structs::Image {
    let cur_images: Vec<structs::Image> = layer.images.clone();

    let weights: Vec<i32> = layer.images
    .into_iter()
    .map(|image: structs::Image| image.rarity.value)
    .collect();

    let dist: WeightedIndex<i32> = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();

    cur_images[dist.sample(&mut rng)].clone()
}

#[wasm_bindgen]
pub fn generate(constructed_layers: &JsValue) {
    info!("[gen-rs] generator started");

    let constructed_layers_input: Vec<structs::Layer> = constructed_layers.into_serde().unwrap();

    let images_chosen: Vec<structs::Image> = constructed_layers_input
    .into_iter()
    .map(|layer: structs::Layer| randomized_image(layer))
    .collect();

    debug!("{:?}", images_chosen);

    // debug!("{:?}", constructed_layers_input);

    info!("[gen-rs] generator finished");
}