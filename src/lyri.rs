use std::{collections::HashMap, sync::Arc};

use files::file_manager::{FileManager, SongsFileManager};
use genius::{
    genius::Genius,
    model::{artist::PrimaryArtist, hit::Hit},
};
use processing::filters;
use scraper::scraper::AppScraper;
use tokio::sync::{mpsc, Mutex};

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

use {crate::args::Args, serde_json::json};

pub async fn lyri(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let genius = Genius::new();

    let hits = genius.search(&args.artist).await?;

    let (artist_id, artist_name) = find_arg_artist_from_hits(&args.artist, hits);

    let artist = genius.artists(artist_id).await?;

    let songs_response = genius.artists_songs(artist.id).await?;

    let processed_songs = filters::artist_songs(artist_id, songs_response);

    let file_json = json!({
        "total": processed_songs.len(),
        "songs": processed_songs
    });

    let file_path = format!("data/{}.json", artist_name.to_lowercase().replace(" ", "_"));

    SongsFileManager::write(file_path.as_str(), file_json.to_string());

    let res_file = SongsFileManager::read(file_path.as_str());

    let res_copy = res_file.clone();

    let (sender, mut receiver) = mpsc::channel::<(u32, String)>(res_file.songs.len());
    let sender = Arc::new(Mutex::new(sender));

    for song in res_file.songs {
        let sender = Arc::clone(&sender);
        tokio::spawn(async move {
            let scraper = AppScraper::new();
            match scraper.from_url(&song.url).await {
                Ok(lyrics) => {
                    let sender = sender.lock().await;
                    match sender.send((song.id, lyrics)).await {
                        Ok(res) => res,
                        Err(err) => panic!("{}", err.to_string()),
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        });
    }

    let mut lyrics_map: HashMap<u32, String> = HashMap::new();

    while let Some((song_id, lyrics)) = receiver.recv().await {
        lyrics_map.insert(song_id, lyrics);
        if lyrics_map.len() == res_copy.songs.len() {
            break;
        }
    }

    let file_data_with_lyrics = res_copy.to_file_data_with_lyrics(lyrics_map);

    let file_json = json!({
        "total": file_data_with_lyrics.songs.len(),
        "songs": file_data_with_lyrics
    });

    let file_path = format!(
        "data/{}_with_lyrics.json",
        artist_name.to_lowercase().replace(" ", "_")
    );
    SongsFileManager::write(file_path.as_str(), file_json.to_string());

    return Ok(());
}
