use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub alternate_names: Vec<String>,
    pub api_path: String,
    pub facebook_name: Option<String>,
    pub followers_count: u32,
    pub header_image_url: String,
    pub id: u32,
    pub image_url: String,
    pub instagram_name: Option<String>,
    pub is_meme_verified: bool,
    pub is_verified: bool,
    pub name: String,
    pub translation_artist: bool,
    pub twitter_name: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrimaryArtist {
    pub id: u32,
    pub name: String,
}
