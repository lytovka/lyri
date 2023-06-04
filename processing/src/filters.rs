use genius::model::song::ArtistSong;
use regex::Regex;

trait PostProcessor {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong>;
}

struct UnknownLanguage;

impl PostProcessor for UnknownLanguage {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        songs
            .into_iter()
            .filter(|song| song.language.is_some())
            .collect()
    }
}

struct IncompleteLyrics;

impl PostProcessor for IncompleteLyrics {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        songs
            .into_iter()
            .filter(|song| song.lyrics_state == "complete")
            .collect()
    }
}

struct MainArtist {
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

struct UnknownReleaseDate;

impl PostProcessor for UnknownReleaseDate {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        songs
            .into_iter()
            .filter(|song| song.release_date_for_display.is_some())
            .collect()
    }
}

struct TitleSanitizer;

impl PostProcessor for TitleSanitizer {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        let re = Regex::new(r"(?i)unreleased|remix|(instrumental)").unwrap();
        songs
            .into_iter()
            .filter(|song| !re.is_match(&song.title.to_lowercase()))
            .collect()
    }
}

pub fn artist_songs(artist_id: u32, mut artist_songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
    let filters: Vec<Box<dyn PostProcessor>> = vec![
        Box::new(UnknownLanguage),
        Box::new(IncompleteLyrics),
        Box::new(UnknownReleaseDate),
        Box::new(MainArtist { artist_id }),
        Box::new(TitleSanitizer),
    ];

    artist_songs = filters
        .into_iter()
        .fold(artist_songs, |songs, post_processor| {
            post_processor.process(songs)
        });

    artist_songs
}
