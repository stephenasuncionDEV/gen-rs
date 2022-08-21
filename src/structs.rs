use serde::{Serialize, Deserialize};

// Rust
#[derive(Serialize, Deserialize, Debug)]
pub struct Rarity {
    pub value: i32,
    pub max: i32,
    pub percentage: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub src: Vec<u8>,
    pub name: String,
    pub image_type: String,
    pub rarity: Rarity
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    pub name: String,
    pub images: Vec<Image>
}

// Client
#[derive(Serialize, Deserialize, Debug)]
pub struct Attributes {
    pub name: String,
    pub images: Vec<Image>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageInput {
    pub name: String,
    pub src: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputLayers {
    pub name: String,
    pub images: Vec<ImageInput>
}