use crate::song::Song;
use reqwest::Client;
use serde::Deserialize;

const BASE_URL: &str = "https://api.genius.com";

struct GeniusEndpoints<'a> {
    search: &'a str,
}

const ENDPOINTS: GeniusEndpoints<'static> = GeniusEndpoints { search: "search" };

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

    pub async fn search(&self, q: &str) -> Result<Vec<Hit>, reqwest::Error> {
        let request = self
            .reqwest
            .get(format!("{}/{}?q={}", BASE_URL, ENDPOINTS.search, q))
            .bearer_auth(&self.token)
            .send()
            .await?;
        let res = request.json::<Response>().await;
        match res {
            Ok(res) => Ok(res.response.hits.unwrap()),
            Err(e) => panic!("Problem returning the result {:?}", e),
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
}

#[derive(Deserialize, Debug)]
pub struct Hit {
    pub index: String,
    pub r#type: String,
    pub result: Song,
}
