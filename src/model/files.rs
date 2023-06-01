use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::song::{ArtistSong, ArtistSongWithLyrics};

#[derive(Deserialize, Debug, Clone)]
pub struct FileData {
    pub total: usize,
    pub songs: Vec<ArtistSong>,
}

impl FileData {
    pub fn to_file_data_with_lyrics(&self, lyrics: HashMap<u32, String>) -> FileDataWithLyrics {
        FileDataWithLyrics {
            total: self.total,
            songs: self
                .songs
                .iter()
                .map(|song| match lyrics.get(&song.id) {
                    Some(lyrics) => song.to_artist_song_with_lyrics(lyrics.to_owned()),
                    None => song.to_artist_song_with_lyrics(String::from("")),
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct FileDataWithLyrics {
    pub total: usize,
    pub songs: Vec<ArtistSongWithLyrics>,
}
