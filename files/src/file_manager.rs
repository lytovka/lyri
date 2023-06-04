use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
    str,
};

use genius::model::song::{ArtistSong, ArtistSongWithLyrics};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

pub trait FileManager<T> {
    fn read(path: &str) -> T;
    fn write(path: &str, content: Value);
}

pub struct SongsFileManager;

impl FileManager<FileData> for SongsFileManager {
    fn read(path: &str) -> FileData {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    fn write(path: &str, content: Value) {
        let mut file = File::create(path).unwrap();
        file.write_all(content.to_string().as_bytes()).unwrap();
    }
}
