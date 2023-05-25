#![allow(dead_code)]

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
    model::{artist::PrimaryArtist, files::FileData, hit::Hit},
    post_processor::{
        IncompleteLyrics, MainArtist, PostProcessor, TitleSanitizer, UnknownLanguage,
        UnknownReleaseDate,
    },
    serde_json::json,
    std::{
        fs::File,
        io::{BufReader, Write},
    },
    tokio,
};

fn find_arg_artist_from_hits(arg_artist: &str, genius_hits: Vec<Hit>) -> (u32, String) {
    let matched_hit = genius_hits
        .iter()
        .find(|&hit| hit.result.primary_artist.name.to_lowercase() == arg_artist.to_lowercase());

    let PrimaryArtist { id, name } = match matched_hit {
        Some(hit) => hit.result.primary_artist.clone(),
        None => panic!("Could not find artist `{}` in Genius hits.", arg_artist),
    };

    (id, name)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let genius = Genius::new();

    let hits = genius.search(&args.artist).await?;

    let (artist_id, artist_name) = find_arg_artist_from_hits(&args.artist, hits);

    let artist = genius.artists(artist_id).await?;

    let mut songs_response = genius.artists_songs(artist.id).await?;

    let post_processors: Vec<Box<dyn PostProcessor>> = vec![
        Box::new(UnknownLanguage),
        Box::new(IncompleteLyrics),
        Box::new(UnknownReleaseDate),
        Box::new(MainArtist {
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
