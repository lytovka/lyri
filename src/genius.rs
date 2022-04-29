use crate::artists::Artist;
use crate::song::Song;
use reqwest::Client;
use serde::Deserialize;

const BASE_URL: &str = "https://api.genius.com";

struct GeniusEndpoints<'a> {
    search: &'a str,
    songs: &'a str,
    artists: &'a str,
}

const ENDPOINTS: GeniusEndpoints<'static> = GeniusEndpoints {
    search: "search",
    songs: "songs",
    artists: "artists",
};

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
    /// The search capability covers all content hosted on Genius (all songs).
    ///
    /// Reference: https://docs.genius.com/#/search-h2
    pub async fn search(&self, q: &str) -> Result<Vec<Hit>, reqwest::Error> {
        let request = self
            .reqwest
            .get(format!("{}/{}?q={}", BASE_URL, ENDPOINTS.search, q))
            .bearer_auth(&self.token)
            .send()
            .await?;
        let res = request.json::<Response>().await;
        match res {
            Ok(res) => match res.meta.status {
                200 => Ok(res.response.hits.unwrap()),
                _ => panic!("Bad status code: {}", res.meta.status),
            },
            Err(e) => panic!("Problem returning the result:\n{:#?}", e),
        }
    }

    /// A song is a document hosted on Genius. It's usually music lyrics.
    /// Data for a song includes details about the document itself and information about all the referents that are attached to it, including the text to which they refer.
    ///
    /// Reference:  https://docs.genius.com/#songs-h2
    pub async fn songs(&self, id: u32) -> Result<Song, reqwest::Error> {
        let request = self
            .reqwest
            .get(format!("{}/{}/{}", BASE_URL, ENDPOINTS.songs, id))
            .bearer_auth(&self.token)
            .send()
            .await?;
        let res = request.json::<Response>().await;
        match res {
            Ok(res) => match res.meta.status {
                200 => Ok(res.response.song.unwrap()),
                _ => panic!("Bad status code: {}", res.meta.status),
            },
            Err(e) => panic!("Problem returning the result:\n{:#?}", e),
        }
    }

    pub async fn artists(&self, id: u32) -> Result<Artist, reqwest::Error> {
        let request = self
            .reqwest
            .get(format!("{}/{}/{}", BASE_URL, ENDPOINTS.artists, id))
            .bearer_auth(&self.token)
            .send()
            .await?;
        let res = request.json::<Response>().await;
        match res {
            Ok(res) => match res.meta.status {
                200 => Ok(res.response.artist.unwrap()),
                _ => panic!("Bad status code: {}", res.meta.status),
            },
            Err(e) => panic!("Problem returning the result:\n{:#?}", e),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Meta {
    status: u32,
}

#[derive(Deserialize, Debug)]
struct Response {
    meta: Meta,
    response: BlobResponse,
}

#[derive(Deserialize, Debug)]
struct BlobResponse {
    hits: Option<Vec<Hit>>,
    song: Option<Song>,
    artist: Option<Artist>,
}

#[derive(Deserialize, Debug)]
pub struct Hit {
    pub index: String,
    pub r#type: String,
    pub result: Song,
}
