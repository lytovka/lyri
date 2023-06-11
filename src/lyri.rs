use std::{collections::HashMap, sync::Arc};

use cli::cli::{Cli, Commands};
use files::file_manager::{FileManager, SongsFileManager};
use genius::{
    genius::Genius,
    model::{artist::PrimaryArtist, hit::Hit, song::ArtistSong},
};
use log::error;
use processing::filters;
use scraper::scraper::AppScraper;
use tokio::sync::Semaphore;

const MAX_PERMITS: usize = 50;

fn file_path_from_artist(artist: &str) -> String {
    format!("data/{}.json", artist.to_lowercase().replace(" ", "_"))
}

fn file_path_with_lyrics_from_artist(artist: &str) -> String {
    format!(
        "data/{}_with_lyrics.json",
        artist.to_lowercase().replace(" ", "_")
    )
}

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

async fn scrape_lyrics_in_parallel(songs: Vec<ArtistSong>) -> HashMap<u32, String> {
    let progress_bar = Arc::new(cli::progress::scrape_progress_bar(songs.len() as u16));
    let semaphore = Arc::new(Semaphore::new(std::cmp::min(songs.len(), MAX_PERMITS)));

    let mut join_handles = Vec::new();
    for song in songs.clone() {
        let pbc = Arc::clone(&progress_bar);
        let semaphore = Arc::clone(&semaphore);
        let _permit = semaphore.acquire_owned().await.unwrap();

        join_handles.push(tokio::spawn(async move {
            match AppScraper::new().from_url(&song.url).await {
                Ok(lyrics) => {
                    drop(_permit);
                    pbc.inc(1);
                    (song.id, lyrics)
                }
                Err(err) => {
                    error!("Error scraping lyrics for song `{}`: {}", song.id, err);
                    (song.id, String::new())
                }
            }
        }));
    }

    let mut lyrics_map: HashMap<u32, String> = HashMap::new();

    for join_handle in join_handles {
        let (song_id, lyrics) = join_handle.await.unwrap();
        lyrics_map.insert(song_id, lyrics);
    }

    lyrics_map
}

use serde_json::json;

pub async fn lyri(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let genius = Genius::new();

    match cli.commands {
        Commands::Artist(args) => {
            let hits = genius.search(&args.name).await?;
            let (artist_id, artist_name) = find_arg_artist_from_hits(&args.name, hits);
            let songs_response = genius.artists_songs(artist_id).await?;
            let filtered_songs = filters::artist_songs(artist_id, songs_response);

            let file_json = json!({
                "total": filtered_songs.len(),
                "songs": filtered_songs
            });

            let file_path = file_path_from_artist(artist_name.as_str());

            SongsFileManager::write(file_path.as_str(), file_json);

            let res_file =
                SongsFileManager::read(file_path_from_artist(artist_name.as_str()).as_str());

            let lyrics_map = scrape_lyrics_in_parallel(res_file.songs.clone()).await;

            let file_data_with_lyrics = res_file.to_file_data_with_lyrics(lyrics_map);

            let file_json = json!({
                "total": file_data_with_lyrics.songs.len(),
                "songs": file_data_with_lyrics
            });

            let file_path = file_path_with_lyrics_from_artist(artist_name.as_str());
            SongsFileManager::write(file_path.as_str(), file_json);
        }
    }

    return Ok(());
}
