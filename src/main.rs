#![allow(dead_code)]
mod args;
mod genius;
mod model;
use {
    crate::genius::Genius,
    args::Args,
    clap::Parser,
    serde_json::json,
    std::{fs::File, io::Write},
    tokio,
};

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

    let songs_response = genius.artists_songs(artist.id).await?;

    let file_path = format!("data/{}.json", artist_name.to_lowercase().replace(" ", "_"));

    let mut file = File::create(file_path).expect("Unable to create file");

    let file_json = json!({
        "total": songs_response.len(),
        "songs": songs_response
    });

    file.write_all(file_json.to_string().as_bytes())
        .expect("could not safe songs to file");

    Ok(())
}
