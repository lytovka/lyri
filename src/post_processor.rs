use regex::Regex;

use crate::model::song::ArtistSong;

pub trait PostProcessor {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong>;
}

pub struct UnknownLanguage;

impl PostProcessor for UnknownLanguage {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        songs
            .into_iter()
            .filter(|song| song.language.is_some())
            .collect()
    }
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

pub struct MainArtist {
    pub artist_name: String,
}

impl PostProcessor for MainArtist {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        songs
            .into_iter()
            .filter(|song| {
                song.primary_artist.name.to_lowercase() == self.artist_name.to_lowercase()
            })
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

pub struct TitleSanitizer;

impl PostProcessor for TitleSanitizer {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        let re = Regex::new(r"(?i)unreleased|remix|(instrumental)").unwrap();
        songs
            .into_iter()
            .filter(|song| !re.is_match(&song.title.to_lowercase()))
            .collect()
    }
}
