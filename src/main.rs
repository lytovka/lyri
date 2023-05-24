#![allow(dead_code)]

use model::song::{ArtistSong, ArtistSongWithLyrics};
use serde::{Deserialize, Serialize};
mod args;
mod genius;
mod model;
mod post_processor;
mod scraper;
use {
    crate::genius::Genius,
    crate::scraper::AppScraper,
    args::Args,
    clap::Parser,
    post_processor::{
        IncompleteLyrics, PostProcessor, PrimaryArtist, TitleSanitizer, UnknownLanguage,
        UnknownReleaseDate,
    },
    serde_json::json,
    std::{
        fs::File,
        io::{BufReader, Write},
    },
    tokio,
};

#[derive(Deserialize, Debug)]
struct FileData {
    total: usize,
    songs: Vec<ArtistSong>,
}

impl FileData {
    fn to_file_data_with_lyrics(&self, lyrics: Vec<String>) -> FileDataWithLyrics {
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
struct FileDataWithLyrics {
    total: usize,
    songs: Vec<ArtistSongWithLyrics>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let genius = Genius::new();

    let hits = genius.search(&args.artist).await?;

    let matched_hit = hits
        .iter()
        .find(|&hit| hit.result.primary_artist.name.to_lowercase() == args.artist.to_lowercase());

    let matched_hit = match matched_hit {
        Some(hit) => hit,
        None => panic!("Could not find any artist by `{}`", args.artist),
    };

    let (artist_id, artist_name) = (
        matched_hit.result.primary_artist.id,
        matched_hit.result.primary_artist.name.clone(),
    );

    let artist = genius.artists(artist_id).await?;

    let mut songs_response = genius.artists_songs(artist.id).await?;

    let post_processors: Vec<Box<dyn PostProcessor>> = vec![
        Box::new(UnknownLanguage),
        Box::new(IncompleteLyrics),
        Box::new(UnknownReleaseDate),
        Box::new(PrimaryArtist {
            artist_name: artist_name.clone(),
        }),
        Box::new(TitleSanitizer),
    ];

    for post_processor in post_processors {
        songs_response = post_processor.process(songs_response);
    }

    let file_json = json!({
        "total": songs_response.len(),
        "songs": songs_response
    });

    let file_path = format!("data/{}.json", artist_name.to_lowercase().replace(" ", "_"));

    let mut file = File::create(&file_path).expect("Unable to create file");
    file.write_all(file_json.to_string().as_bytes())
        .expect("could not safe songs to file");

    let file = File::open(file_path)?;
    let reader = BufReader::new(&file);
    let res_file: FileData = serde_json::from_reader(reader).unwrap();

    let scraper = AppScraper::new();

    let mut lyric_vec: Vec<String> = Vec::new();

    for song in &res_file.songs {
        let lyrics = scraper.from_url(&song.url).await?;
        lyric_vec.push(lyrics);
    }

    let file_data_with_lyrics = res_file.to_file_data_with_lyrics(lyric_vec);

    let file_json = json!({
        "total": res_file.songs.len(),
        "songs": file_data_with_lyrics
    });

    let mut file_with_lyrics: File =
        File::create("data/tinariwen_with_lyrics.json").expect("Unable to create file");

    file_with_lyrics
        .write_all(file_json.to_string().as_bytes())
        .expect("could not safe songs to file");

    Ok(())
}
