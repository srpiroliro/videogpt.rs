use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use std::fmt;

const API_URL: &str = "https://api.supadata.ai/v1/youtube/transcript";
const DEFAULT_LANG: &str = "en";

type TranscriptContent = String;

#[derive(Debug, Deserialize)]
pub struct SupdataResponse {
    pub content: TranscriptContent,
    // pub lang: String,

    // #[serde(rename = "availableLangs")]
    // pub available_langs: Vec<String>,
}

// #[derive(Debug, Deserialize)]
// pub struct TranscriptItem {
//     pub text: String,
//     pub offset: i64,
//     pub duration: i64,
//     pub lang: String,
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Native,
    Generate,
    Auto,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Native => write!(f, "native"),
            Mode::Generate => write!(f, "generate"),
            Mode::Auto => write!(f, "auto"),
        }
    }
}

pub struct Supdata {
    client: reqwest::Client,
}

impl Supdata {
    pub fn new(api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(&api_key).unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn get_transcript(
        &self,
        video_url: &str,
        // lang: Option<String>, // ignored on default mode
        // mode: Option<Mode>,
    ) -> anyhow::Result<TranscriptContent> {
        // let lang = lang.unwrap_or(DEFAULT_LANG.to_string());
        // let mode = mode.unwrap_or(Mode::Auto);

        let response = self
            .client
            .get(API_URL)
            .query(&[("url", video_url)])
            .query(&[("text", true)])
            // .query(&[("lang", lang)])
            // .query(&[("mode", mode.to_string())])
            .send()
            .await?; // throws error if request fails

        let body = response.text().await?; // throws error if response is not valid text
        let response: SupdataResponse = serde_json::from_str(&body)?; // throws error if response is not valid SupdataResponse

        Ok(response.content)
    }
}
