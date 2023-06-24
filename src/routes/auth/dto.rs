use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JwtTokenResponse {
    pub token: String,
}

impl JwtTokenResponse {
    pub fn new(token: String) -> Self {
        JwtTokenResponse { token }
    }
}
