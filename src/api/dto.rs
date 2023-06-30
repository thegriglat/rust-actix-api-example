use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LinkDto {
    pub url: String,
}

impl LinkDto {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct UrlRequest {
    #[validate(url(message = "Value must be url"))]
    pub url: String,
}
