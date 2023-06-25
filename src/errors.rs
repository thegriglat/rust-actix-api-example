use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new<S>(error_message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            message: error_message.into(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError::new(value.to_string())
    }
}

#[derive(Serialize, Debug)]
pub struct ValidationErrorResponse {
    pub messages: HashMap<String, Vec<String>>,
}

impl From<&validator::ValidationErrors> for ValidationErrorResponse {
    fn from(error: &validator::ValidationErrors) -> Self {
        let error_fields = error.field_errors();

        let error_map: HashMap<String, Vec<String>> = error_fields
            .iter()
            .map(|keyvalue| {
                (
                    keyvalue.0.to_string(),
                    keyvalue
                        .1
                        .iter()
                        .map(|field_error| match &field_error.message {
                            Some(v) => v.to_string(),
                            None => "Validation error".to_string(),
                        })
                        .collect(),
                )
            })
            .collect();

        ValidationErrorResponse {
            messages: error_map,
        }
    }
}
