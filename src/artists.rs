use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
/// Non-exhaustive return model for GET [`/artists/:id`](https://docs.genius.com/#artists-h2)
pub struct Artist {
    pub alternate_names: Vec<String>,
    pub api_path: String,
    // description omitted
    pub facebook_name: Option<String>,
    pub followers_count: u32,
    pub header_image_url: String,
    /// The ID for this artist (for use in GET `/artists/:id` requests)
    pub id: u32,
    /// URL for an image to display for this artist
    pub image_url: String,
    pub instagram_name: Option<String>,
    pub is_meme_verified: bool,
    pub is_verified: bool,
    /// The full name of this artist
    pub name: String,
    pub translation_artist: bool,
    pub twitter_name: Option<String>,
    pub url: String,
}
