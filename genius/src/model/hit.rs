use serde::{Serialize, Deserialize};

use super::song::ArtistSong;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hit {
    pub index: String,
    pub r#type: String,
    pub result: ArtistSong,
}