use scraper::{Html, Selector};

pub struct AppScraper {
    client: reqwest::Client,
}

impl AppScraper {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        let request = self.client.get(url).send().await?;

        match request.status() {
            reqwest::StatusCode::OK => Ok(self.get_lyrics(&request.text().await?)),
            bad_status_code => panic!("Bad status code: {}", bad_status_code),
        }
    }

    fn get_lyrics(&self, html: &str) -> String {
        let document = Html::parse_document(html);
        let selector = Selector::parse(r#"div[data-lyrics-container="true"]"#).unwrap();
        let lyrics = document.select(&selector).next().unwrap();
        println!("{:#?}", lyrics.inner_html());
        lyrics.inner_html()
    }
}
