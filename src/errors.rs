use serde::Serialize;

#[derive(Serialize)]
pub struct AppError {
    error: String,
}

impl AppError {
    pub fn new(error_message: &str) -> Self {
        Self {
            error: error_message.to_string(),
        }
    }
}
