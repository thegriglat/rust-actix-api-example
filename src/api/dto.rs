use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct LinkDto {
    pub url: String,
}

impl LinkDto {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Deserialize, Validate)]
pub struct UrlRequest {
    #[validate(url(message = "Value must be url"))]
    pub url: String,
}
