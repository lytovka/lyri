use serde::{Deserialize, Serialize};

use super::song::{ArtistSong, ArtistSongWithLyrics};

#[derive(Deserialize, Debug)]
pub struct FileData {
    pub total: usize,
    pub songs: Vec<ArtistSong>,
}

impl FileData {
    pub fn to_file_data_with_lyrics(&self, lyrics: Vec<String>) -> FileDataWithLyrics {
        FileDataWithLyrics {
            total: self.total,
            songs: self
                .songs
                .iter()
                .zip(lyrics)
                .map(|(song, lyrics)| song.to_artist_song_with_lyrics(lyrics))
                .collect(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct FileDataWithLyrics {
    pub total: usize,
    pub songs: Vec<ArtistSongWithLyrics>,
}
