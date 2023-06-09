use log::{error, info};

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

const BASE_URL: &str = "https://api.genius.com";
const PER_PAGE: u16 = 50;
const GENIUS_ACCESS_TOKEN_ENV_VAR: &str = "GENIUS_ACCESS_TOKEN";

#[derive(Deserialize, Debug)]
struct Response<T> {
    response: T,
}

pub struct Genius {
    reqwest: Client,
    auth_token: String,
}

impl Genius {
    pub fn new() -> Self {
        Self {
            auth_token: dotenv::var(GENIUS_ACCESS_TOKEN_ENV_VAR).expect(
                format!(
                    "Could not find environment variable `{}`. Make sure it's set in the `.env` file.",
                    GENIUS_ACCESS_TOKEN_ENV_VAR
                )
                .as_str(),
            ),
            reqwest: Client::new(),
        }
    }

    /// https://docs.genius.com/#/search-h2
    pub async fn search(&self, q: &str) -> Result<Vec<Hit>, Error> {
        let request = self
            .reqwest
            .get(format!("{}/search", BASE_URL))
            .query(&[("q", q)])
            .bearer_auth(&self.auth_token)
            .send()
            .await;

        match request {
            Ok(res) => match res.error_for_status() {
                Ok(res_ok) => match res_ok.json::<Response<SearchResponse>>().await {
                    Ok(search_res) => Ok(search_res.response.hits.unwrap()),
                    Err(err) => {
                        error!("Error while deserializing to SearchResponse: {:#?}", err);
                        Err(err)
                    }
                },
                Err(res_err) => {
                    error!("Bad status code: {:?}", res_err.status());
                    Err(res_err)
                }
            },
            Err(err) => {
                error!("Unexpected result: {}", err);
                Err(err)
            }
        }
    }

    /// https://docs.genius.com/#songs-h2
    pub async fn songs(&self, id: u32) -> Result<ArtistSong, Error> {
        let response = self
            .reqwest
            .get(format!("{}/songs/{}", BASE_URL, id))
            .bearer_auth(&self.auth_token)
            .send()
            .await;

        match response {
            Ok(res) => match res.error_for_status() {
                Ok(res_ok) => match res_ok.json::<Response<SongResponse>>().await {
                    Ok(song_res) => Ok(song_res.response.song.unwrap()),
                    Err(err) => {
                        error!("Error while deserializing to SongResponse: {:#?}", err);
                        Err(err)
                    }
                },
                Err(err) => Err(err),
            },
            Err(err) => {
                error!("Bad status code: {:?}", err.status());
                Err(err)
            }
        }
    }

    /// https://docs.genius.com/#artists-h2
    pub async fn artists(&self, id: u32) -> Result<Artist, Error> {
        let response = self
            .reqwest
            .get(format!("{}/artists/{}", BASE_URL, id))
            .bearer_auth(&self.auth_token)
            .send()
            .await;

        match response {
            Ok(res) => match res.error_for_status() {
                Ok(res_ok) => match res_ok.json::<Response<ArtistResponse>>().await {
                    Ok(artist_res) => Ok(artist_res.response.artist.unwrap()),
                    Err(err) => {
                        error!("Error while deserializing to SongResponse: {:#?}", err);
                        Err(err)
                    }
                },
                Err(err) => {
                    error!("Bad status code: {:?}", err.status());
                    Err(err)
                }
            },
            Err(err) => {
                error!("Bad status code: {:?}", err.status());
                Err(err)
            }
        }
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
                .query(&[("page", page), ("per_page", PER_PAGE)])
                .bearer_auth(&self.auth_token)
                .send()
                .await?;

            match response.error_for_status() {
                Ok(res_ok) => match res_ok.json::<Response<ArtistSongsResponse>>().await {
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
}
