use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Song {
    /// Number of annotations on this song
    annotation_count: u32,
    /// Path (not full URL) to this song through Genius API
    api_path: String,
    apple_music_id: Option<String>,
    apple_music_player_url: Option<String>,
    artist_names: String,
    // description: omitted
    embed_content: Option<String>,
    featured_video: Option<bool>,
    full_title: String,
    header_image_thumbnail_url: String,
    header_image_url: String,
    /// This song's ID
    id: u32,
    lyrics_owner_id: u32,
    lyrics_state: String,
    /// url path to lyrics page
    path: String,
    pyongs_count: u32,
    release_date: Option<String>,
    release_date_for_display: Option<String>,
    song_art_image_thumbnail_url: String,
    song_art_image_url: String,
    // stats omitted
    /// Name of this song
    title: String,
    title_with_featured: String,
    /// Full URL to this song on genius.com
    url: String,
}
