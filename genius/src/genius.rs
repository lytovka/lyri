use log::{error, info};
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
        let mut total_count: usize = 0;
        let mut songs_res: Vec<ArtistSong> = vec![];
        let spinner = cli::progress::fetch_progress_bar();

        loop {
            let response = self
                .reqwest
                .get(format!("{}/artists/{}/songs", BASE_URL, artist_id))
                .query(&[("page", page), ("per_page", PER_PAGE_PARAM)])
                .bearer_auth(&self.auth_token)
                .send()
                .await?;

            match response.error_for_status() {
                Ok(res_ok) => match res_ok.json::<MyResponse<ArtistSongsResponse>>().await {
                    Ok(res_parsed) => match res_parsed.response.songs {
                        Some(songs) => {
                            if songs.is_empty() {
                                spinner.finish_and_clear();
                                break Ok(songs_res);
                            }
                            total_count += songs.len();
                            songs_res.extend(songs);
                            page += 1;
                        }
                        None => {
                            if !songs_res.is_empty() {
                                spinner.finish_and_clear();
                                break Ok(songs_res);
                            } else {
                                error!("No songs have been returned");
                            }
                        }
                    },
                    Err(e) => error!("Error while deserializing to ArtistSongsResponse: {:#?}", e),
                },
                Err(res_bad) => {
                    if !songs_res.is_empty() {
                        info!("Returning {} songs", total_count);
                        spinner.finish_and_clear();
                        break Ok(songs_res);
                    } else {
                        error!("Bad status code {:?}", res_bad.status());
                    }
                }
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
                self.handle_single_item_response(item_res.response)
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
                self.handle_multiple_items_response(item_res.response)
            }
            Err(res_err) => {
                error!("Bad status code: {:?}", res_err.status());
                Err(res_err)
            }
        }
    }

    fn handle_multiple_items_response<T>(&self, response: T) -> Result<Vec<T::Item>, Error>
    where
        T: ResponseMultipleItems,
    {
        match response.get_items() {
            Some(items) => Ok(items),
            None => panic!("No items have been returned"),
        }
    }

    fn handle_single_item_response<T>(&self, response: T) -> Result<T::Item, Error>
    where
        T: ResponseSingleItem,
    {
        match response.get_item() {
            Some(item) => Ok(item),
            None => panic!("No item has been returned"),
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

