mod args;
mod file_manager;
mod post_processor;

use file_manager::{FileManager, SongsFileManager};
use genius::{
    genius::Genius,
    model::{artist::PrimaryArtist, hit::Hit, song::ArtistSong},
};
use scraper::scraper::AppScraper;
use {
    args::Args,
    clap::Parser,
    post_processor::{
        IncompleteLyrics, MainArtist, PostProcessor, TitleSanitizer, UnknownLanguage,
        UnknownReleaseDate,
    },
    serde_json::json,
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

fn process_artist_songs(artist_id: u32, mut artist_songs: Vec<ArtistSong>) -> Vec<ArtistSong> {
    let post_processors: Vec<Box<dyn PostProcessor>> = vec![
        Box::new(UnknownLanguage),
        Box::new(IncompleteLyrics),
        Box::new(UnknownReleaseDate),
        Box::new(MainArtist { artist_id }),
        Box::new(TitleSanitizer),
    ];

    for post_processor in post_processors {
        artist_songs = post_processor.process(artist_songs);
    }

    artist_songs
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let genius = Genius::new();

    let hits = genius.search(&args.artist).await?;

    let (artist_id, artist_name) = find_arg_artist_from_hits(&args.artist, hits);

    let artist = genius.artists(artist_id).await?;

    let songs_response = genius.artists_songs(artist.id).await?;

    let processed_songs = process_artist_songs(artist_id, songs_response);

    let file_json = json!({
        "total": processed_songs.len(),
        "songs": processed_songs
    });

    let file_path = format!("data/{}.json", artist_name.to_lowercase().replace(" ", "_"));

    SongsFileManager::write(file_path.as_str(), file_json.to_string());

    let res_file = SongsFileManager::read(file_path.as_str());

    let scraper = AppScraper::new();

    let mut lyric_vec: Vec<String> = vec![];

    for song in &res_file.songs {
        let lyrics = scraper.from_url(&song.url).await?;
        lyric_vec.push(lyrics);
    }

    let file_data_with_lyrics = res_file.to_file_data_with_lyrics(lyric_vec);

    let file_json = json!({
        "total": file_data_with_lyrics.songs.len(),
        "songs": file_data_with_lyrics
    });

    let file_path = format!(
        "data/{}_with_lyrics.json",
        artist_name.to_lowercase().replace(" ", "_")
    );
    SongsFileManager::write(file_path.as_str(), file_json.to_string());

    Ok(())
}
