use std::{collections::HashMap, sync::Arc, time::Duration};

use files::file_manager::{FileManager, SongsFileManager};
use genius::{
    genius::Genius,
    model::{artist::PrimaryArtist, hit::Hit, song::ArtistSong},
};
use log::{error, warn};
use processing::filters;
use scraper::scraper::AppScraper;
use tokio::{
    sync::{mpsc, Mutex},
    time::timeout,
};

const IDLE_TIMEOUT: Duration = Duration::from_secs(2);

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
    let progress_bar = cli::progress::scrape_progress_bar(songs.len() as u16);
    let (sender, mut receiver) = mpsc::channel::<(u32, String)>(songs.len());
    let sender = Arc::new(Mutex::new(sender));

    for song in songs.clone() {
        let sender = Arc::clone(&sender);
        tokio::spawn(async move {
            let scraper = AppScraper::new();
            match scraper.from_url(&song.url).await {
                Ok(lyrics) => match sender.lock().await.send((song.id, lyrics)).await {
                    Ok(res) => res,
                    Err(err) => panic!("{}", err.to_string()),
                },
                Err(err) => error!("Error while sending value to receiver: {}", err),
            }
        });
    }

    let mut lyrics_map: HashMap<u32, String> = HashMap::new();
    loop {
        progress_bar.set_position(lyrics_map.len() as u64);
        let message = timeout(IDLE_TIMEOUT, receiver.recv()).await;

        match message {
            Ok(res) => match res {
                Some((song_id, lyrics)) => {
                    lyrics_map.insert(song_id, lyrics);
                    if lyrics_map.len() == songs.len() {
                        break;
                    }
                }
                None => break,
            },
            Err(_) => {
                // Timeout occurred, no message received within the idle timeout
                warn!("Idle timeout reached");
                break;
            }
        }
    }

    lyrics_map
}

use {crate::args::Args, serde_json::json};

pub async fn lyri(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let genius = Genius::new();

    let hits = genius.search(&args.artist).await?;
    let (artist_id, artist_name) = find_arg_artist_from_hits(&args.artist, hits);
    /* i don't remember why this was needed...
    let artist = genius.artists(artist_id).await?;
    */
    let songs_response = genius.artists_songs(artist_id).await?;
    let filtered_songs = filters::artist_songs(artist_id, songs_response);

    let file_json = json!({
        "total": filtered_songs.len(),
        "songs": filtered_songs
    });

    let file_path = file_path_from_artist(artist_name.as_str());

    SongsFileManager::write(file_path.as_str(), file_json);

    let res_file = SongsFileManager::read(file_path_from_artist(artist_name.as_str()).as_str());

    let lyrics_map = scrape_lyrics_in_parallel(res_file.songs.clone()).await;

    let file_data_with_lyrics = res_file.to_file_data_with_lyrics(lyrics_map);

    let file_json = json!({
        "total": file_data_with_lyrics.songs.len(),
        "songs": file_data_with_lyrics
    });

    let file_path = file_path_with_lyrics_from_artist(artist_name.as_str());
    SongsFileManager::write(file_path.as_str(), file_json);

    return Ok(());
}
