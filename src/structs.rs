use serde::{Serialize, Deserialize};

// Rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rarity {
    pub value: i32,
    pub max: i32,
    pub percentage: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub src: String,
    pub name: String,
    pub image_type: String,
    pub rarity: Rarity
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
pub struct InputImage {
    pub name: String,
    pub src: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputLayer {
    pub name: String,
    pub images: Vec<InputImage>
}