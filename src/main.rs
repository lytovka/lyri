#![allow(dead_code)]
use std::{fs::File, io::Write};
mod genius;
mod model;
use crate::genius::Genius;
use clap::Parser;
use tokio;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    artist: String,
}

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
    let serialized_songs = serde_json::to_string(&songs_response).unwrap();
    let mut file = File::create(format!(
        "data/{}.json",
        artist_name.to_lowercase().replace(" ", "_")
    ))
    .expect("Unable to create file");
    file.write_all(serialized_songs.as_bytes())
        .expect("could not safe songs to file");
}
