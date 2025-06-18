use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;

const DATA_FILE: &str = "data.json";

#[derive(Error, Debug)]
pub enum UrlShortenerError {
    #[error("Short URL not found")]
    UrlNotFound,
    #[error("Invalid URL provided")]
    InvalidUrl,
    #[error("Storage error: {0}")]
    StorageError(String),
}

impl From<std::io::Error> for UrlShortenerError {
    fn from(err: std::io::Error) -> Self {
        UrlShortenerError::StorageError(err.to_string())
    }
}

impl From<serde_json::Error> for UrlShortenerError {
    fn from(err: serde_json::Error) -> Self {
        UrlShortenerError::StorageError(err.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct StoredData {
    urls: HashMap<String, String>,
}

#[derive(Clone)]
pub struct UrlShortener {
    urls: Arc<Mutex<HashMap<String, String>>>,
}

impl UrlShortener {
    pub fn new() -> Result<Self, UrlShortenerError> {
        let urls = if Path::new(DATA_FILE).exists() {
            let mut file = File::open(DATA_FILE)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str::<StoredData>(&contents)?.urls
        } else {
            HashMap::new()
        };

        Ok(UrlShortener {
            urls: Arc::new(Mutex::new(urls)),
        })
    }

    pub fn shorten(&self, url: &str) -> Result<String, UrlShortenerError> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(UrlShortenerError::InvalidUrl);
        }

        let short_code = nanoid::nanoid!(6);
        {
            let mut urls = self.urls.lock();
            urls.insert(short_code.clone(), url.to_string());
        }
        self.save()?;
        Ok(short_code)
    }

    pub fn expand(&self, short_code: &str) -> Result<String, UrlShortenerError> {
        let urls = self.urls.lock();
        urls.get(short_code)
            .map(|url| url.to_string())
            .ok_or(UrlShortenerError::UrlNotFound)
    }

    pub fn list(&self) -> Vec<(String, String)> {
        let urls = self.urls.lock();
        urls.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    fn save(&self) -> Result<(), UrlShortenerError> {
        let urls = self.urls.lock();
        let data = StoredData {
            urls: urls.clone(),
        };
        let json = serde_json::to_string_pretty(&data)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(DATA_FILE)?;

        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
