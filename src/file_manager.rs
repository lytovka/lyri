use std::{
    fs::File,
    io::{BufReader, Write},
    str,
};

use genius::model::song::{ArtistSong, ArtistSongWithLyrics};
use serde::{Deserialize, Serialize};

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

pub trait FileManager<T> {
    fn read(path: &str) -> T;
    fn write(path: &str, content: String);
}

pub struct SongsFileManager;

impl FileManager<FileData> for SongsFileManager {
    fn read(path: &str) -> FileData {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    fn write(path: &str, content: String) {
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }
}
