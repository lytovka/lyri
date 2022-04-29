use dotenv;
pub mod artists;
pub mod genius;
pub mod song;
// use genius_rs::Genius;
use crate::genius::Genius;
use tokio;

#[tokio::main]
async fn main() {
    let genius = Genius::new(
        dotenv::var("GENIUS_ACCESS_TOKEN").expect("Could not find .env var OR the value is wrong"),
    );
    // let response = genius.search("Ariana%20Grande").await;
    // match response {
    //     Ok(r) => {
    //         let song_id = r.get(0).unwrap().result.id;
    //         let song_response = genius.songs(song_id).await;
    //         println!("song response - {:#?}", song_response);
    //     }
    //     Err(r) => println!("{}", r),
    // }

    let response = genius.artists(1421).await;
    match response {
        Ok(r) => {
            // println!("artist response - {:#?}", r);
            let songs_response = genius.artists_songs(r.id).await;
            println!("songs vector - {:#?}", songs_response.unwrap());
        }
        Err(r) => println!("{}", r),
    }
}