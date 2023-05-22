use serde::{Deserialize, Serialize};

use super::artist::PrimaryArtist;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArtistSong {
    pub annotation_count: Option<u32>,
    pub api_path: String,
    pub artist_names: String,
    pub full_title: String,
    pub header_image_thumbnail_url: String,
    pub header_image_url: String,
    pub id: u32,
    pub lyrics_owner_id: Option<u32>,
    pub lyrics_state: String,
    pub path: String,
    pub primary_artist: PrimaryArtist,
    pub pyongs_count: Option<u32>,
    pub release_date_components: Option<String>,
    pub release_date_for_display: Option<String>,
    pub release_date_with_abbreviated_month_for_display: Option<String>,
    pub song_art_image_thumbnail_url: String,
    pub song_art_image_url: String,
    pub title_with_featured: String,
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SongResponse {
    pub song: Option<ArtistSong>,
}
