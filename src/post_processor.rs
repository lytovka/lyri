use genius::model::song::ArtistSong;
use regex::Regex;

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
    pub artist_id: u32,
}

impl PostProcessor for MainArtist {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        songs
            .into_iter()
            .filter(|song| song.primary_artist.id == self.artist_id)
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
