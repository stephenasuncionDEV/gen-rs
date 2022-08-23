use crate::{structs, info, Utc, throw};
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use photon_rs::{PhotonImage, base64_to_image, multiple, to_image_data};
use web_sys::{ImageData, CanvasRenderingContext2d};

pub fn randomized_image(layer: structs::Layer) -> structs::Image {
    let cur_images: Vec<structs::Image> = layer.images.clone();

    let weights: Vec<i32> = layer.images
    .into_iter()
    .map(|image: structs::Image| image.rarity.value)
    .collect();

    let dist: WeightedIndex<i32> = WeightedIndex::new(&weights).unwrap();
    let mut rng: ThreadRng = thread_rng();

    cur_images[dist.sample(&mut rng)].clone()
}

#[wasm_bindgen]
pub fn generate(ctx: &CanvasRenderingContext2d, constructed_layers: &JsValue, generation_size: i32) {
    info!("[gen-rs] generator of {} unique images started", generation_size);

    let start_time: chrono::NaiveTime = Utc::now().time();

    let constructed_layers_input: Vec<structs::Layer> = constructed_layers.into_serde().unwrap();

    let mut possibleCombination: i32 = 1;
    for layer in &constructed_layers_input {
        possibleCombination *= layer.images.len() as i32;
    }

    if possibleCombination < generation_size {
        throw!("[gen-rs] possible combination is under the desired collection count {}/{}. You must add more images to your layer(s).",
        possibleCombination,
        generation_size);
    }

    let images_chosen: Vec<structs::Image> = constructed_layers_input
    .into_iter()
    .map(|layer: structs::Layer| randomized_image(layer))
    .collect();

    let images_chosen_photon: Vec<PhotonImage> = images_chosen
    .into_iter()
    .map(|image: structs::Image| base64_to_image(&image.src[22..]))
    .collect();

    let mut base_image: PhotonImage = images_chosen_photon[0].to_owned();

    for n in 1..images_chosen_photon.len() {
        multiple::watermark(&mut base_image, &images_chosen_photon[n], 0, 0);
    }

    let generated_image: ImageData = to_image_data(base_image);

    ctx.put_image_data(&generated_image, 0.0, 0.0);

    let end_time: chrono::NaiveTime = Utc::now().time();

    let elapsed_time: chrono::Duration = end_time - start_time;

    info!("[gen-rs] generator finished in {}ms", elapsed_time.num_milliseconds());
}