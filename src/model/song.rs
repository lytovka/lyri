use serde::{Deserialize, Serialize};

use super::artist::PrimaryArtist;

#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    pub id: u32,
    pub annotation_count: Option<u32>,
    pub api_path: String,
    pub apple_music_id: Option<String>,
    pub apple_music_player_url: Option<String>,
    pub artist_names: String,
    pub embed_content: Option<String>,
    pub featured_video: Option<bool>,
    pub full_title: String,
    pub header_image_thumbnail_url: String,
    pub header_image_url: String,
    pub lyrics_owner_id: Option<u32>,
    pub lyrics_state: String,
    pub path: String,
    pub pyongs_count: Option<u32>,
    pub release_date: Option<String>,
    pub release_date_for_display: Option<String>,
    pub song_art_image_thumbnail_url: String,
    pub song_art_image_url: String,
    pub title: String,
    pub title_with_featured: String,
    pub url: String,
    pub primary_artist: PrimaryArtist,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SongResponse {
    pub song: Option<Song>,
}
