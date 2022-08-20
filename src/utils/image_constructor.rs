pub use log::info;
use crate::structs;

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