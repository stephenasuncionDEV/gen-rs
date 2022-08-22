use crate::{structs, info, JsValue, try_join_all, JsFuture};
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;

pub async fn src_to_base64(image_src: String, image_type: &String) -> Result<String, JsValue> {
    let window: web_sys::Window = web_sys::window().unwrap();
    let promise: JsFuture = JsFuture::from(window.fetch_with_str(&image_src));
    let result: JsValue = promise.await?;
    let response: web_sys::Response = result.dyn_into().unwrap();
    let image_data: JsValue = JsFuture::from(response.array_buffer()?).await?;
    let image_bytes: Vec<u8> = Uint8Array::new(&image_data).to_vec();
    let base64: String = base64::encode(&*image_bytes);

    Ok(format!("data:{};base64,{}", image_type, base64.replace("\r\n", "")))
}

pub async fn construct_layer_images_data(image: structs::InputImage, size: i32, image_type: &String) -> Result<structs::Image, String> {
    let image_base64: String = src_to_base64(image.src, &image_type).await.unwrap();

    Ok(structs::Image {
        name: image.name,
        src: image_base64,
        image_type: image_type.to_owned(),
        rarity: structs::Rarity {
            max: size * 50,
            value: 50,
            percentage: 100 / size
        }
    })
}

pub async fn construct_layer_images(images: Vec<structs::InputImage>, image_type: &String) -> Result<Vec<structs::Image>, String> {
    info!("[gen-rs] constructing layer images");

    let size: i32 = images.len().clone() as i32;

    let new_images: Vec<structs::Image> = try_join_all(images
        .into_iter()
        .map(|image: structs::InputImage| construct_layer_images_data(image, size, &image_type)))
        .await
        .unwrap();

   Ok(new_images)
}