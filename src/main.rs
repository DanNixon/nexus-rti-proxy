use axum::{Router, extract, http::HeaderValue, response::IntoResponse, routing::get};
use chrono::Utc;
use regex::Regex;
use reqwest::{self, header::AUTHORIZATION};
use scraper::{Html, Selector};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct KeeperOfTheUrls {
    #[serde(rename = "kmlHost")]
    kml_host: String,
    token: String,
}

impl KeeperOfTheUrls {
    async fn new() -> Self {
        let response = reqwest::get("https://metro-rti.nexus.org.uk/MapEmbedded")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let document = Html::parse_document(&response);

        let script_selector = Selector::parse("script").unwrap();
        for script in document.select(&script_selector) {
            let script_content = script.text().collect::<Vec<_>>().join("");
            if script_content.contains("token") {
                let json_regex = Regex::new(r"\{.*?\}").unwrap();
                let captures = json_regex.captures(&script_content).unwrap();
                let json = captures.get(0).unwrap().as_str();
                return serde_json::from_str::<KeeperOfTheUrls>(json).unwrap();
            }
        }

        panic!();
    }

    fn url(&self, file: &str) -> String {
        format!(
            "{}/api/geo/{file}?d={}",
            self.kml_host,
            Utc::now().timestamp_millis()
        )
    }
}

async fn get_kml(extract::Path(file): extract::Path<String>) -> impl IntoResponse {
    let url_keeper = KeeperOfTheUrls::new().await;
    let url = url_keeper.url(&file);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", url_keeper.token)).unwrap(),
        )
        .send()
        .await
        .unwrap();

    response.bytes().await.unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/{file}", get(get_kml));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
