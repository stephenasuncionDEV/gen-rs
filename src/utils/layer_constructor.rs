use crate::{structs, info, try_join_all};
use super::image_constructor::construct_layer_images;

pub async fn construct_layers_data(layer: structs::InputLayers, image_type: &String) -> Result<structs::Layer, String> {
    let images: Vec<structs::Image> = construct_layer_images(layer.images, image_type).await.unwrap();

    Ok(structs::Layer {
        name: layer.name,
        images
    })
}

pub async fn construct_layers(input_layers: Vec<structs::InputLayers>, image_type: String) -> Result<Vec<structs::Layer>, String> {
    info!("[gen-rs] constructing layers");

    let new_layers: Vec<structs::Layer> = try_join_all(input_layers
        .into_iter()
        .map(|layer: structs::InputLayers| construct_layers_data(layer, &image_type)))
        .await
        .unwrap();

    Ok(new_layers)
}