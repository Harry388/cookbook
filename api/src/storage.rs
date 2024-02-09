pub mod dufs;

use poem::{Result, error::InternalServerError};
use poem_openapi::types::multipart::Upload;
use async_trait::async_trait;

pub struct FileData {
    bytes: Vec<u8>,
    ext: String
}

#[async_trait]
pub trait Storage: Sync + Send {

    async fn put_file(&self, path: &str, file: Upload) -> Result<String>;

    async fn get_file(&self, path: &str) -> Result<Vec<u8>>;

    async fn delete_file(&self, path: &str) -> Result<()>;

    async fn format_upload(&self, file: Upload) -> Result<FileData> {
        let ext = match file.file_name() {
            Some(name) => name.split(".").last(),
            None => None
        };
        let ext = match ext {
            Some(e) => e.to_string(),
            None => "".to_string()
        };
        let bytes = file.into_vec().await.map_err(InternalServerError)?;
        Ok(FileData { bytes, ext })
    }
}
