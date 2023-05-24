use {
    once_cell::sync::Lazy,
    reqwest::{Client, Error},
    scraper::{Html, Selector},
};

static LYRIC_SELECTOR: Lazy<Selector> =
    Lazy::new(|| Selector::parse(r#"div[data-lyrics-container="true"]"#).unwrap());

pub struct AppScraper {
    client: Client,
}

impl AppScraper {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn from_url(&self, url: &str) -> Result<String, Error> {
        let request = self.client.get(url).send().await?;

        match request.status() {
            reqwest::StatusCode::OK => {
                let lyrics = self.scrape_lyrics(&request.text().await?);
                Ok(lyrics)
            }
            bad_status_code => panic!("Bad status code: {}", bad_status_code),
        }
    }

    fn scrape_lyrics(&self, html: &str) -> String {
        let document = Html::parse_document(html);
        let lyrics_containers = document.select(&LYRIC_SELECTOR);

        lyrics_containers
            .into_iter()
            .map(|x| x.text().collect::<Vec<_>>().join("\n"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
