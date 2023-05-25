use serde::{Deserialize, Serialize};

use super::{artist::Artist, hit::Hit, song::ArtistSong};

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub hits: Option<Vec<Hit>>,
}

#[derive(Deserialize, Debug)]
pub struct ArtistResponse {
    pub artist: Option<Artist>,
}
#[derive(Deserialize, Debug)]
pub struct ArtistSongsResponse {
    pub songs: Option<Vec<ArtistSong>>,
}

#[derive(Deserialize, Debug)]
pub struct LyricsResponse {
    pub plain: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct SongResponse {
    pub song: Option<ArtistSong>,
}
