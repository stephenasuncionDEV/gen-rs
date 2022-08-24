use crate::{structs, info, Utc, throw, debug};
use super::metadata_constructor::construct_metadata;
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

pub fn get_images_hash(images_chosen: &Vec<structs::Image>) -> String {
    let mut images_name: String = String::from("");
    for image in images_chosen {
        images_name.push_str(&image.name);
    }
    let hash: md5::Digest = md5::compute(&images_name);
    format!("{:x}", hash)
}

pub fn get_generated_image(images_chosen: &Vec<structs::Image>) -> ImageData {
    let images_chosen_photon: Vec<PhotonImage> = images_chosen
    .to_owned()
    .into_iter()
    .map(|image: structs::Image| base64_to_image(&image.src[22..]))
    .collect();

    let mut base_image: PhotonImage = images_chosen_photon[0].to_owned();

    for n in 1..images_chosen_photon.len() {
        multiple::watermark(&mut base_image, &images_chosen_photon[n], 0, 0);
    }

    to_image_data(base_image)
}

#[wasm_bindgen]
pub fn generate(ctx: &CanvasRenderingContext2d, opt: &JsValue) {
    let options: structs::InputGenerate = opt.into_serde().unwrap();

    info!("[gen-rs] generator of {} unique images started", options.size);

    let start_time: chrono::NaiveTime = Utc::now().time();

    let mut possibleCombination: i32 = 1;
    for layer in &options.layers {
        possibleCombination *= layer.images.len() as i32;
    }

    if possibleCombination < options.size {
        throw!("[gen-rs] possible combination is under the desired collection count {}/{}. You must add more images to your layer(s).",
        possibleCombination,
        options.size);
    }

    let mut hash_list: Vec<String> = vec![];
    let mut start_count: i32 = options.startCount;
    let mut render_index: i32 = 1;

    while render_index < options.size {

        let images_chosen: Vec<structs::Image> = options.layers
        .to_owned()
        .into_iter()
        .map(|layer: structs::Layer| randomized_image(layer))
        .collect();

        let current_hash: String = get_images_hash(&images_chosen);
        
        if !hash_list.contains(&current_hash) {
            hash_list.push(current_hash);

            let generated_image: ImageData = get_generated_image(&images_chosen);

            ctx.put_image_data(&generated_image, 0.0, 0.0);

            start_count += 1;
            render_index += 1;
        }
    }

    let end_time: chrono::NaiveTime = Utc::now().time();

    let elapsed_time: chrono::Duration = end_time - start_time;

    info!("[gen-rs] generator finished in {}ms", elapsed_time.num_milliseconds());
}