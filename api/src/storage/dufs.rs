use reqwest::Client;
use poem::{Result, error::InternalServerError};
use poem_openapi::types::multipart::Upload;
use async_trait::async_trait;

use crate::storage::Storage;

#[derive(Clone)]
pub struct DufsStorage {
    base_url: String
}

impl DufsStorage {

    pub fn new(base_url: &str) -> DufsStorage {
        DufsStorage { base_url: String::from(base_url) }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", &self.base_url, path)
    }

}

#[async_trait]
impl Storage for DufsStorage {

    async fn put_file(&self, path: &str, file: Upload) -> Result<String> {
        let file_data = Self::format_upload(file).await?;
        let path = format!("{}.{}", path, file_data.ext);
        let full_path = self.url(&path);
        let client = Client::new();
        client
            .put(full_path)
            .body(file_data.bytes)
            .send()
            .await
            .map_err(InternalServerError)?;
        Ok(path)
    }

}

