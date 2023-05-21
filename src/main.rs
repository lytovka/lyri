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
async fn main() {
    let args = Args::parse();

    let genius = Genius::new();

    let hits = genius
        .search(&args.artist)
        .await
        .unwrap_or_else(|_| panic!("Could not find any hits of this artist."));
    let matched_hit = hits
        .iter()
        .find(|hit| hit.result.primary_artist.name.to_lowercase() == args.artist.to_lowercase())
        .unwrap_or_else(|| panic!("Could not find an artist by `--name` argument."));

    let (artist_id, artist_name) = (
        matched_hit.result.primary_artist.id,
        matched_hit.result.primary_artist.name.clone(),
    );

    let artist = genius
        .artists(artist_id)
        .await
        .unwrap_or_else(|_| panic!("Could not find any artist by this id."));

    let songs_response = genius
        .artists_songs(artist.id)
        .await
        .expect("unable to generate songs");

    let file_path = format!("data/{}.json", artist_name.to_lowercase().replace(" ", "_"));

    let mut file = File::create(file_path).expect("Unable to create file");

    let file_json = json!({
        "total": songs_response.len(),
        "songs": songs_response
    });

    file.write_all(file_json.to_string().as_bytes())
        .expect("could not safe songs to file");
}
