use serde::{Deserialize, Serialize};

use super::artist::PrimaryArtist;

#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    pub id: u32,
    annotation_count: Option<u32>,
    api_path: String,
    apple_music_id: Option<String>,
    apple_music_player_url: Option<String>,
    artist_names: String,
    embed_content: Option<String>,
    featured_video: Option<bool>,
    full_title: String,
    header_image_thumbnail_url: String,
    header_image_url: String,
    lyrics_owner_id: Option<u32>,
    lyrics_state: String,
    path: String,
    pyongs_count: Option<u32>,
    release_date: Option<String>,
    release_date_for_display: Option<String>,
    song_art_image_thumbnail_url: String,
    song_art_image_url: String,
    title: String,
    title_with_featured: String,
    url: String,
    primary_artist: PrimaryArtist,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SongResponse {
    pub song: Option<Song>,
}
