use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LinkDto {
    pub url: String,
}

impl LinkDto {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Deserialize)]
pub struct UrlRequest {
    pub url: String,
}
