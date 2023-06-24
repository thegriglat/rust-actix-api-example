use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

pub struct AppState {
    links: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            links: Mutex::new(HashMap::new()),
        }
    }

    pub fn add(self: &Self, url: String) -> String {
        let mut data = self.links.lock().expect("Cannot get access to mutex");
        let short_url = Uuid::new_v4().to_string();
        data.insert(short_url.to_string(), url.clone());
        short_url
    }

    pub fn get(self: &Self, short_url: &String) -> Option<String> {
        let data = self.links.lock().expect("Cannot get access to mutex");
        let value = data.get(short_url);
        match value {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}
