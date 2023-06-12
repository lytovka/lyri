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

struct TitleSanitizer {
    pattern: String,
}

impl PostProcessor for TitleSanitizer {
    fn process(&self, songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
        let re = Regex::new(&format!(r"(?i){}", self.pattern)).unwrap();
        songs
            .into_iter()
            .filter(|song| !re.is_match(&song.title.to_lowercase()))
            .collect()
    }
}

pub struct FilterOptions {
    pub include_features: Option<bool>,
    pub antipattern: Option<String>,
}

pub fn apply(artist_id: u32, artist_songs: Vec<ArtistSong>, options: FilterOptions) -> Vec<ArtistSong> {
    let mut filters: Vec<Box<dyn PostProcessor>> = vec![
        Box::new(UnknownLanguage),
        Box::new(IncompleteLyrics),
        Box::new(UnknownReleaseDate),
        Box::new(MainArtist { artist_id }),
    ];

    if let Some(feat) = options.include_features {
        if feat {
            filters.pop();
        }
    }
    if let Some(antipattern) = options.antipattern {
        let p = if !antipattern.is_empty() {
            antipattern
        } else {
            String::from("unreleased|remix|(instrumental)")
        };
        filters.push(Box::new(TitleSanitizer { pattern: p }));
    }

    filters
        .into_iter()
        .fold(artist_songs, |songs, post_processor| {
            post_processor.process(songs)
        })
}
