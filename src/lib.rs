use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use thiserror::Error;
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
struct StoredData {
    urls: HashMap<String, String>,
}

#[derive(Clone)] 
pub struct UrlShortener {
    urls: HashMap<String, String>,
}

impl UrlShortener {
    pub fn new() -> Result<Self, UrlShortenerError> {
        let urls = if Path::new(DATA_FILE).exists() {
            let mut file = File::open(DATA_FILE)
                .map_err(|e| UrlShortenerError::StorageError(e.to_string()))?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|e| UrlShortenerError::StorageError(e.to_string()))?;
            serde_json::from_str::<StoredData>(&contents)
                .map_err(|e| UrlShortenerError::StorageError(e.to_string()))?
                .urls
        } else {
            HashMap::new()
        };

        Ok(UrlShortener { urls })
    }

    pub fn shorten(&mut self, url: &str) -> Result<String, UrlShortenerError> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(UrlShortenerError::InvalidUrl);
        }

        let short_code = nanoid::nanoid!(6);
        self.urls.insert(short_code.clone(), url.to_string());
        self.save()?;
        Ok(short_code)
    }

    pub fn expand(&self, short_code: &str) -> Result<String, UrlShortenerError> {
        self.urls
            .get(short_code)
            .map(|url| url.to_string())
            .ok_or(UrlShortenerError::UrlNotFound)
    }

    pub fn list(&self) -> Vec<(&String, &String)> {
        self.urls.iter().collect()
    }

    fn save(&self) -> Result<(), UrlShortenerError> {
        let data = StoredData {
            urls: self.urls.clone(),
        };
        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| UrlShortenerError::StorageError(e.to_string()))?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(DATA_FILE)
            .map_err(|e| UrlShortenerError::StorageError(e.to_string()))?;

        file.write_all(json.as_bytes())
            .map_err(|e| UrlShortenerError::StorageError(e.to_string()))?;
        Ok(())
    }
}
