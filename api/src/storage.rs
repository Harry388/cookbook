pub mod dufs;

use poem::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    async fn put_file(&self, path: &str, file_data: Vec<u8>) -> Result<()>;
}