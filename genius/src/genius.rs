use log::{error};
use reqwest::Response;
use serde::de::DeserializeOwned;

use {
    crate::model::{
        artist::Artist,
        hit::Hit,
        responses::{ArtistResponse, ArtistSongsResponse, SearchResponse, SongResponse},
        song::ArtistSong,
    },
    reqwest::{Client, Error},
    serde::Deserialize,
};

const PER_PAGE_PARAM: u16 = 50;
const BASE_URL: &str = "https://api.genius.com";
const GENIUS_ACCESS_TOKEN_ENV_VAR: &str = "GENIUS_ACCESS_TOKEN";

#[derive(Deserialize, Debug)]
struct MyResponse<T> {
    response: T,
}

pub struct Genius {
    reqwest: Client,
    auth_token: String,
}

impl Genius {
    pub fn new() -> Self {
        Self {
            auth_token: dotenv::var(GENIUS_ACCESS_TOKEN_ENV_VAR).unwrap_or_else(|_| 
                panic!( "Could not find environment variable `{}`. Make sure it is declared in the `.env` file.", GENIUS_ACCESS_TOKEN_ENV_VAR),
            ),
            reqwest: Client::new(),
        }
    }

    /// https://docs.genius.com/#/search-h2
    pub async fn search(&self, q: &str) -> Result<Vec<Hit>, Error> {
        let response = self
            .reqwest
            .get(format!("{}/search", BASE_URL))
            .query(&[("q", q)])
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

       self.handle_vector_response::<SearchResponse>(response).await 
    }

    /// https://docs.genius.com/#songs-h2
    pub async fn songs(&self, id: u32) -> Result<ArtistSong, Error> {
        let response = self
            .reqwest
            .get(format!("{}/songs/{}", BASE_URL, id))
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        self.handle_response::<SongResponse>(response).await
    }

    /// https://docs.genius.com/#artists-h2
    pub async fn artists(&self, id: u32) -> Result<Artist, Error> {
        let response = self
            .reqwest
            .get(format!("{}/artists/{}", BASE_URL, id))
            .bearer_auth(&self.auth_token)
            .send()
            .await?;

        self.handle_response::<ArtistResponse>(response).await        
    }

    /// https://docs.genius.com/#artists-h2
    pub async fn artists_songs(&self, artist_id: u32) -> Result<Vec<ArtistSong>, Error> {
        let mut page: u16 = 1;
        let mut all_songs: Vec<ArtistSong> = vec![];
        let spinner = cli::progress::fetch_progress_bar();

        loop {
            let response = self
                .reqwest
                .get(format!("{}/artists/{}/songs", BASE_URL, artist_id))
                .query(&[("page", page), ("per_page", PER_PAGE_PARAM)])
                .bearer_auth(&self.auth_token)
                .send()
                .await?;

            let current_page_songs = self.handle_vector_response::<ArtistSongsResponse>(response).await?;
            
            if current_page_songs.is_empty() {
                spinner.finish_and_clear();
                break Ok(all_songs);
            }
            else {
                all_songs.extend(current_page_songs);
                page += 1;
            }
        }
    }

    async fn handle_response<T>(&self, res: Response) -> Result<T::Item, Error>
    where
        T: DeserializeOwned + ResponseSingleItem,
    {
        match res.error_for_status() {
            Ok(res_ok) => {
                let item_res = res_ok.json::<MyResponse<T>>().await?;
                match item_res.response.get_item() {
                    Some(item) => Ok(item),
                    None => panic!("No item has been returned"),
                }
            }
            Err(res_err) => {
                error!("Bad status code: {:?}", res_err.status());
                Err(res_err)
            }
        }
    }

    async fn handle_vector_response<T>(&self, res: Response) -> Result<Vec<T::Item>, Error>
    where
        T: DeserializeOwned + ResponseMultipleItems,
    {
        match res.error_for_status() {
            Ok(res_ok) => {
                let item_res = res_ok.json::<MyResponse<T>>().await?;
                match item_res.response.get_items() {
                    Some(items) => Ok(items),
                    None => panic!("No items have been returned"),
                }
            }
            Err(res_err) => {
                error!("Bad status code: {:?}", res_err.status());
                Err(res_err)
            }
        }
    }
    
}


trait ResponseMultipleItems {
    type Item;
    fn get_items(self) -> Option<Vec<Self::Item>>;
}

impl ResponseMultipleItems for SearchResponse {
    type Item = Hit;
    fn get_items(self) -> Option<Vec<Self::Item>> {
        self.hits
    }
}

impl ResponseMultipleItems for ArtistSongsResponse {
    type Item = ArtistSong;
    fn get_items(self) -> Option<Vec<Self::Item>> {
        self.songs
    }
}

trait ResponseSingleItem {
    type Item;
    fn get_item(self) -> Option<Self::Item>;
}

impl ResponseSingleItem for SongResponse {
    type Item = ArtistSong;
    fn get_item(self) -> Option<Self::Item> {
        self.song
    }
}

impl ResponseSingleItem for ArtistResponse {
    type Item = Artist;
    fn get_item(self) -> Option<Self::Item> {
        self.artist
    }
}

