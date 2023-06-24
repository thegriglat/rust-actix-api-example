use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShortenedLink {
    pub url: String,
}

impl ShortenedLink {
    pub fn new(url: String) -> Self {
        ShortenedLink { url }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UrlRequest {
    pub url: String,
}

impl UrlRequest {
    pub fn new(url: String) -> Self {
        UrlRequest {
            url: format!("/{}", url),
        }
    }
}
