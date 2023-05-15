use dotenv;
use std::{fs::File, io::Write};
pub mod genius;
use crate::genius::Genius;
use tokio;

#[tokio::main]
async fn main() {
    let genius = Genius::new(
        dotenv::var("GENIUS_ACCESS_TOKEN").expect("Could not find .env var OR the value is wrong"),
    );

    let response = genius.artists(21100).await;
    match response {
        Ok(r) => {
            let songs_response = genius
                .artists_songs(r.id)
                .await
                .expect("unable to generate songs");
            let serialized_songs = serde_json::to_string(&songs_response).unwrap();
            let mut file = File::create("data/oasis.json").expect("Unable to create file");
            file.write_all(serialized_songs.as_bytes())
                .expect("could not safe songs to file");
        }
        Err(r) => println!("{}", r),
    }
}
