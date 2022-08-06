use crate::artists::Artist;
use crate::song::Song;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct Genius {
    reqwest: Client,
    token: String,
}

impl Genius {
    pub fn new(token: String) -> Self {
        Self {
            reqwest: Client::new(),
            token,
        }
    }
    /// GET `/search`
    ///  
    /// The search capability covers all content hosted on Genius (all songs).
    ///
    /// Reference: https://docs.genius.com/#/search-h2
    pub async fn search(&self, q: &str) -> Result<Vec<Hit>, reqwest::Error> {
        let request = self
            .reqwest
            .get("https://api.genius.com/search")
            .query(&[("q", q)])
            .bearer_auth(&self.token)
            .send()
            .await?;

        match request.status() {
            reqwest::StatusCode::OK => match request.json::<Response<SearchResponse>>().await {
                Ok(res) => Ok(res.response.hits.unwrap()),
                Err(e) => panic!("Unexpected result:\n{:#?}", e),
            },
            bad_status_code => panic!("Bad status code: {}", bad_status_code),
        }
    }

    /// GET `/songs/:id`
    ///
    /// A song is a document hosted on Genius. It's usually music lyrics.
    /// Data for a song includes details about the document itself and information about all the referents that are attached to it, including the text to which they refer.
    ///
    /// Reference:  https://docs.genius.com/#songs-h2
    pub async fn songs(&self, id: u32) -> Result<Song, reqwest::Error> {
        let response = self
            .reqwest
            .get(format!("https://api.genius.com/songs/{}", id))
            .bearer_auth(&self.token)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<Response<SongResponse>>().await {
                Ok(res) => Ok(res.response.song.unwrap()),
                Err(e) => panic!("Unexpected result:\n{:#?}", e),
            },
            bad_status_code => panic!("Bad status code: {}", bad_status_code),
        }
    }

    /// GET `/artists/:id`
    ///
    /// Data for a specific artist.
    ///
    /// https://docs.genius.com/#artists-h2
    pub async fn artists(&self, id: u32) -> Result<Artist, reqwest::Error> {
        let response = self
            .reqwest
            .get(format!("https://api.genius.com/artists/{}", id))
            .bearer_auth(&self.token)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<Response<ArtistResponse>>().await {
                Ok(res) => Ok(res.response.artist.unwrap()),
                Err(e) => panic!("Unexpected result:\n{:#?}", e),
            },
            bad_status_code => panic!("Bad status code: {}", bad_status_code),
        }
    }

    /// GET `/artists/:id/songs`
    ///
    /// Documents (songs) for the artist specified. By default, 20 items are returned for each request.
    ///
    /// https://docs.genius.com/#artists-h2
    pub async fn artists_songs(&self, id: u32) -> Result<Vec<Song>, reqwest::Error> {
        let mut page_count: u16 = 1;
        let mut resulting_vector: Vec<Song> = vec![];

        loop {
            let response = self
                .reqwest
                .get(format!("https://api.genius.com/artists/{}/songs", id,))
                .query(&[("page", page_count), ("per_page", 30)])
                .bearer_auth(&self.token)
                .send()
                .await?;

            match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<Response<ArtistSongsResponse>>().await {
                        Ok(res) => match res.response.songs {
                            Some(songs) => {
                                if songs.is_empty() {
                                    break Ok(resulting_vector);
                                }
                                resulting_vector.extend(songs);
                                page_count += 1;
                                println!("Parsing page: {}\n", page_count);
                            }
                            None => {
                                if !resulting_vector.is_empty() {
                                    break Ok(resulting_vector);
                                } else {
                                    panic!("No song has been returned");
                                }
                            }
                        },
                        Err(e) => panic!("Deserialization error :\n{:#?}", e),
                    }
                }
                bad_status_code => {
                    if !resulting_vector.is_empty() {
                        break Ok(resulting_vector);
                    } else {
                        panic!("Bad status code {}", bad_status_code);
                    }
                }
            }
        }
    }

    pub async fn lyrics(&self, id: u32) -> Result<Vec<String>, reqwest::Error> {
        let response = self
            .reqwest
            .get(format!("https://lyrics.altart.tk/api/lyrics/{}", id))
            .bearer_auth(&self.token)
            .send()
            .await;

        let response = match response {
            Ok(response) => response.json::<LyricsResponse>().await,
            Err(e) => return Err(e),
        };

        let plain = match response {
            Ok(res) => res.plain,
            Err(e) => return Err(e),
        };

        match plain {
            Some(text) => Ok(text.split("\n").map(String::from).collect::<Vec<String>>()),
            None => panic!("blah"),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    response: T,
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    hits: Option<Vec<Hit>>,
}
#[derive(Deserialize, Debug, Serialize)]
struct SongResponse {
    song: Option<Song>,
}
#[derive(Deserialize, Debug)]
struct ArtistResponse {
    artist: Option<Artist>,
}
#[derive(Deserialize, Debug)]
struct ArtistSongsResponse {
    songs: Option<Vec<Song>>,
}

#[derive(Deserialize, Debug)]
pub struct Hit {
    pub index: String,
    pub r#type: String,
    pub result: Song,
}

#[derive(Deserialize, Debug)]
struct LyricsResponse {
    plain: Option<String>,
}
