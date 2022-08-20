use crate::JsValue;
use crate::info;
use crate::structs;
use js_sys::Uint8Array;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;

pub fn construct_layer_images(images: Vec<structs::ImageInput>, image_type: &String) -> Vec<structs::Image> {
    info!("[gen-rs] constructing layer images");

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

pub async fn load_image(url: String) -> Result<Uint8Array, JsValue> {
    let window: web_sys::Window = web_sys::window().unwrap();
    let promise: JsFuture = JsFuture::from(window.fetch_with_str(&url));
    let result: JsValue = promise.await?;
    let response: web_sys::Response = result.dyn_into().unwrap();
    let image_data: JsValue = JsFuture::from(response.array_buffer()?).await?;
    Ok(Uint8Array::new(&image_data))
}