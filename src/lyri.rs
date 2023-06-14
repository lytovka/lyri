use cli::cli::{ArtistArgs, Cli, Commands};
use files::file_manager::{FileManager, SongsFileManager};
use genius::{
    genius::{ArtistSongsOptions, Genius, SongsSort},
    model::{artist::PrimaryArtist, hit::Hit, song::ArtistSong},
};
use log::{error, info};
use processing::filters::{self, FilterOptions};
use scraper::scraper::AppScraper;
use serde_json::json;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::Semaphore;

const MAX_PERMITS: usize = 50;

fn build_path(artist: &str, dir_path: Option<String>) -> PathBuf {
    let mut pb = PathBuf::new();
    if let Some(dir_path) = dir_path {
        pb.push(dir_path);
    }
    pb.push(artist.to_lowercase().replace(" ", "_"));
    pb.set_extension("json");
    info!("Path: {:?}", pb);
    pb
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

fn to_songs_sort_type(sort: Option<String>) -> Option<SongsSort> {
    let sort = sort.unwrap_or(String::new());

    match sort.as_str() {
        "popularity" => Some(SongsSort::Popularity),
        "title" => Some(SongsSort::Title),
        _ => None,
    }
}

pub async fn lyri(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let genius = Genius::new();

    match cli.commands {
        Commands::Artist(ArtistArgs {
            name,
            limit,
            antipattern,
            features,
            sort,
            output,
        }) => {
            let hits = genius.search(&name).await?;
            let (artist_id, artist_name) = find_arg_artist_from_hits(&name, hits);
            let songs_response = genius
                .artists_songs(
                    artist_id,
                    ArtistSongsOptions {
                        sort: to_songs_sort_type(sort),
                    },
                )
                .await?;
            let mut filtered_songs = filters::apply(
                artist_id,
                songs_response,
                FilterOptions {
                    include_features: features,
                    antipattern: antipattern,
                },
            );
            if let Some(l) = limit {
                if l < filtered_songs.len() as u32 {
                    filtered_songs.truncate(l as usize);
                }
            }
            let file_json = json!({
                "total": filtered_songs.len(),
                "songs": filtered_songs
            });

            let path_buf = build_path(artist_name.as_str(), output);
            let _ = SongsFileManager::try_write(path_buf.as_path(), file_json);
            let res_file = SongsFileManager::read(path_buf.as_path());
            let lyrics_map = scrape_lyrics_in_parallel(res_file.songs.clone()).await;
            let file_data_with_lyrics = res_file.to_file_data_with_lyrics(lyrics_map);
            let file_json = json!({
                "total": file_data_with_lyrics.songs.len(),
                "songs": file_data_with_lyrics
            });
            SongsFileManager::write(path_buf.as_path(), file_json);
        }
    }

    return Ok(());
}
