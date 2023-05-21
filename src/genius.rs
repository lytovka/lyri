use {
    crate::model::{
        artist::Artist,
        hit::Hit,
        responses::{ArtistResponse, ArtistSongsResponse, SearchResponse},
        song::{Song, SongResponse},
    },
    reqwest::Client,
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
    /// GET `/search`
    ///  
    /// The search capability covers all content hosted on Genius (all songs).
    ///
    /// Reference: https://docs.genius.com/#/search-h2
    pub async fn search(&self, q: &str) -> Result<Vec<Hit>, reqwest::Error> {
        let url: String = format!("{}/search", self.base_url);

        let request = self
            .reqwest
            .get(url)
            .query(&[("q", q)])
            .bearer_auth(&self.auth_token)
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
            .bearer_auth(&self.auth_token)
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
            .bearer_auth(&self.auth_token)
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
    pub async fn artists_songs(&self, artist_id: u32) -> Result<Vec<Song>, reqwest::Error> {
        let mut page: u16 = 1;
        let mut total_count: usize = 0;
        let mut resulting_vector: Vec<Song> = vec![];

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
                                if songs.is_empty(){
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
