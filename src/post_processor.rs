use regex::Regex;

use crate::model::song::ArtistSong;

pub trait PostProcessor {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong>;
}

pub struct IncompleteLyrics;

impl PostProcessor for IncompleteLyrics {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        songs
            .into_iter()
            .filter(|song| song.lyrics_state == "complete")
            .collect()
    }
}

pub struct UnknownReleaseDate;

impl PostProcessor for UnknownReleaseDate {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        songs
            .into_iter()
            .filter(|song| song.release_date_for_display.is_some())
            .collect()
    }
}

pub struct UnreleasedSongs;

impl PostProcessor for UnreleasedSongs {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        let re = Regex::new(r"(?i)unreleased").unwrap();
        songs
            .into_iter()
            .filter(|song| !re.is_match(&song.title.to_lowercase()))
            .collect()
    }
}
