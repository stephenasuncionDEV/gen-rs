use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::ImageData;
use js_sys::Uint8Array;
use log::{Level, info, debug};
mod structs;

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(Level::Debug).ok();
}

#[wasm_bindgen]
pub async fn load_image(url: String) -> Result<Uint8Array, JsValue> {
    let window: web_sys::Window = web_sys::window().unwrap();
    let promise: JsFuture = JsFuture::from(window.fetch_with_str(&url));
    let result: JsValue = promise.await?;
    let response: web_sys::Response = result.dyn_into().unwrap();
    let image_data: JsValue = JsFuture::from(response.array_buffer()?).await?;
    Ok(Uint8Array::new(&image_data))
}

pub fn construct_layer_images(images: Vec<structs::ImageInput>, image_type: &String) -> Vec<structs::Image> {
    let size: i32 = images.len().clone() as i32;
    images
   .into_iter()
   .map(|image: structs::ImageInput| structs::Image {
        name: image.name,
        src: image.src,
        image_type: image_type.to_owned(),
        rarity: structs::Rarity {
            max: size * 50,
            value: 50,
            percentage: 100 / size
        }
   })
   .collect()
}

#[wasm_bindgen]
pub fn construct_layers(layers: &JsValue, image_type: String) {
    info!("[gen-rs] constructing layers");

    let layers_input_vec: Vec<structs::LayersInput> = layers.into_serde().unwrap();

    let new_layers_input: Vec<structs::Layer> = layers_input_vec
    .into_iter()
    .map(|layer: structs::LayersInput| structs::Layer {
        name: layer.name,
        images: construct_layer_images(layer.images, &image_type.clone())
    })
    .collect();

    debug!("{:?}", new_layers_input);

    info!("[gen-rs] constructed layers");
}

// #[wasm_bindgen]
// pub fn contruct(layers: &JsValue) -> JsValue {
//     info!("[gen-rs] Constructing Layers");

//     let layers: Vec<structs::Layers> = layers.into_serde().unwrap();

//     for x in &layers {
//         let name = x.name.clone();
        
//         debug!("{}", name);
//     }

//     info!("[gen-rs] Layers Constructed");
// }

#[wasm_bindgen]
pub fn generate(layers: &JsValue) {
    info!("[gen-rs] Generator Started");

    

    info!("[gen-rs] Generator Finished");
}