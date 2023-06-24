use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

use crate::errors::AppError;

pub struct AppState {
    links: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            links: Mutex::new(HashMap::new()),
        }
    }

    pub fn add(&self, url: String) -> Result<String, AppError> {
        let mut data = self
            .links
            .lock()
            .map_err(|_| AppError::new("Cannot get lock "))?;
        let short_url = Uuid::new_v4().to_string();
        data.insert(short_url.to_string(), url);
        Ok(short_url)
    }

    pub fn get(&self, short_url: &String) -> Result<String, AppError> {
        let data = self
            .links
            .lock()
            .map_err(|_| AppError::new("Cannot get lock"))?;
        let value = data.get(short_url);
        match value {
            Some(value) => Ok(value.clone()),
            None => Err(AppError::new("Link not found")),
        }
    }
}
