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
    /// GET `/search`
    ///  
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
        let res = request.json::<Response<SearchResponse>>().await;
        match res {
            Ok(res) => match res.meta.status {
                200 => Ok(res.response.hits.unwrap()),
                _ => panic!("Bad status code: {}", res.meta.status),
            },
            Err(e) => panic!("Problem returning the result:\n{:#?}", e),
        }
    }

    /// GET `/songs/:id`
    ///
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
        let res = request.json::<Response<SongResponse>>().await;
        match res {
            Ok(res) => match res.meta.status {
                200 => Ok(res.response.song.unwrap()),
                _ => panic!("Bad status code: {}", res.meta.status),
            },
            Err(e) => panic!("Problem returning the result:\n{:#?}", e),
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
            .get(format!("{}/{}/{}", BASE_URL, ENDPOINTS.artists, id))
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
        let response = self
            .reqwest
            .get(format!(
                "{}/{}/{}/{}",
                BASE_URL, ENDPOINTS.artists, id, ENDPOINTS.songs
            ))
            .bearer_auth(&self.token)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<Response<ArtistSongsResponse>>().await
            {
                Ok(res) => Ok(res.response.songs.unwrap()),
                Err(e) => panic!("Unexpected result:\n{:#?}", e),
            },
            bad_status_code => panic!("Bad status code: {}", bad_status_code),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Meta {
    status: u32,
}

#[derive(Deserialize, Debug)]
struct Response<T> {
    meta: Meta,
    response: T,
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    hits: Option<Vec<Hit>>,
}
#[derive(Deserialize, Debug)]
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
    next_page: Option<u16>,
}

#[derive(Deserialize, Debug)]
pub struct Hit {
    pub index: String,
    pub r#type: String,
    pub result: Song,
}
