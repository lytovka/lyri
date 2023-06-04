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

const PER_PAGE: u16 = 50;

#[derive(Deserialize, Debug)]
struct Response<T> {
    response: T,
}

pub struct Genius<'a> {
    reqwest: Client,
    auth_token: String,
    base_url: &'a str,
}

impl Genius<'_> {
    pub fn new() -> Self {
        Self {
            auth_token: dotenv::var("GENIUS_ACCESS_TOKEN")
                .expect("Could not find .env var OR the value is wrong"),
            base_url: "https://api.genius.com",
            reqwest: Client::new(),
        }
    }

    /// https://docs.genius.com/#/search-h2
    pub async fn search(&self, q: &str) -> Result<Vec<Hit>, Error> {
        let request = self
            .reqwest
            .get(format!("{}/search", self.base_url))
            .query(&[("q", q)])
            .bearer_auth(&self.auth_token)
            .send()
            .await;

        match request {
            Ok(res) => match res.error_for_status() {
                Ok(res_ok) => match res_ok.json::<Response<SearchResponse>>().await {
                    Ok(search_res) => Ok(search_res.response.hits.unwrap()),
                    Err(err) => {
                        println!("Error while deserializing to SearchResponse: {:#?}", err);
                        Err(err)
                    }
                },
                Err(res_err) => {
                    println!("Bad status code: {:?}", res_err.status());
                    Err(res_err)
                }
            },
            Err(err) => {
                println!("Unexpected result: {}", err);
                Err(err)
            }
        }
    }

    /// https://docs.genius.com/#songs-h2
    pub async fn songs(&self, id: u32) -> Result<ArtistSong, Error> {
        let response = self
            .reqwest
            .get(format!("https://api.genius.com/songs/{}", id))
            .bearer_auth(&self.auth_token)
            .send()
            .await;

        match response {
            Ok(res) => match res.error_for_status() {
                Ok(res_ok) => match res_ok.json::<Response<SongResponse>>().await {
                    Ok(song_res) => Ok(song_res.response.song.unwrap()),
                    Err(err) => {
                        println!("Error while deserializing to SongResponse: {:#?}", err);
                        Err(err)
                    }
                },
                Err(err) => {
                    println!("Bad status code: {:?}", err.status());
                    Err(err)
                }
            },
            Err(e) => panic!("Unexpected result:\n{:#?}", e),
        }
    }

    /// https://docs.genius.com/#artists-h2
    pub async fn artists(&self, id: u32) -> Result<Artist, Error> {
        let response = self
            .reqwest
            .get(format!("https://api.genius.com/artists/{}", id))
            .bearer_auth(&self.auth_token)
            .send()
            .await;

        match response {
            Ok(res) => match res.error_for_status() {
                Ok(res_ok) => match res_ok.json::<Response<ArtistResponse>>().await {
                    Ok(artist_res) => Ok(artist_res.response.artist.unwrap()),
                    Err(err) => {
                        println!("Error while deserializing to SongResponse: {:#?}", err);
                        Err(err)
                    }
                },
                Err(err) => {
                    println!("Bad status code: {:?}", err.status());
                    Err(err)
                }
            },
            Err(e) => panic!("Unexpected result:\n{:#?}", e),
        }
    }

    /// https://docs.genius.com/#artists-h2
    pub async fn artists_songs(&self, artist_id: u32) -> Result<Vec<ArtistSong>, Error> {
        let mut page: u16 = 1;
        let mut total_count: usize = 0;
        let mut resulting_vector: Vec<ArtistSong> = vec![];

        loop {
            let response = self
                .reqwest
                .get(format!(
                    "https://api.genius.com/artists/{}/songs",
                    artist_id
                ))
                .query(&[("page", page), ("per_page", PER_PAGE)])
                .bearer_auth(&self.auth_token)
                .send()
                .await?;

            println!("Parsing page: {}\n", page);

            match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<Response<ArtistSongsResponse>>().await {
                        Ok(res) => match res.response.songs {
                            Some(songs) => {
                                if songs.is_empty() {
                                    break Ok(resulting_vector);
                                }
                                total_count += songs.len();
                                resulting_vector.extend(songs);
                                page += 1;
                            }
                            None => {
                                if !resulting_vector.is_empty() {
                                    println!("Returning {} songs", total_count);
                                    break Ok(resulting_vector);
                                } else {
                                    panic!("No song has been returned");
                                }
                            }
                        },
                        Err(e) => panic!("Unexpected result:\n{:#?}", e),
                    }
                }
                bad_status_code => {
                    if !resulting_vector.is_empty() {
                        println!("Returning {} songs", total_count);
                        break Ok(resulting_vector);
                    } else {
                        panic!("Bad status code {:?}", bad_status_code);
                    }
                }
            }
        }
    }
}
