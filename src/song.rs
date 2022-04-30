use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Song {
    /// Number of annotations on this song
    annotation_count: Option<u32>,
    /// Path (not full URL) to this song through Genius API
    api_path: String,
    apple_music_id: Option<String>,
    apple_music_player_url: Option<String>,
    artist_names: String,
    embed_content: Option<String>,
    featured_video: Option<bool>,
    full_title: String,
    header_image_thumbnail_url: String,
    header_image_url: String,
    /// This song's ID
    pub id: u32,
    lyrics_owner_id: Option<u32>,
    lyrics_state: String,
    /// url path to lyrics page
    path: String,
    /// number of "pyongs" (indication of user interest) this song has received.
    pyongs_count: Option<u32>,
    release_date: Option<String>,
    release_date_for_display: Option<String>,
    song_art_image_thumbnail_url: String,
    song_art_image_url: String,
    /// Name of this song
    title: String,
    title_with_featured: String,
    /// Full URL to this song's page on genius.com
    url: String,
    /// The main artist to which this song is attributed
    primary_artist: PrimaryArtist,
}

#[derive(Deserialize, Debug)]
pub struct PrimaryArtist {
    pub id: u32,
    pub name: String,
}
