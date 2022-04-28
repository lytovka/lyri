use dotenv;
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
    let response = genius.search("Ariana%20Grande").await;
    match response {
        Ok(r) => println!("res main - {:#?}", r),
        Err(r) => println!("{}", r),
    }
}
