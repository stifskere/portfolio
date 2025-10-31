use async_trait::async_trait;

use crate::utils::external::storage::interface::{GenericError, StorageInterface};

/// This handler connects to amazon S3 compatible
/// servers.
pub struct S3Handler {
    bucket: String,
    ip: String,
    region: Option<String>,
    base_directory: String
}

impl S3Handler {
    pub(super) async fn connect(url: &str) -> Result<Box<dyn StorageInterface>, GenericError> {
        todo!()
    }
}

#[async_trait]
impl StorageInterface for S3Handler {
    async fn get(&self, path: &str) -> Result<Option<Vec<u8>>, GenericError> {
        todo!()
    }

    async fn put(&self, path: &str, content: Vec<u8>) -> Result<(), GenericError> {
        todo!()
    }

    async fn delete(&self, path: &str) -> Result<(), GenericError> {
        todo!()
    }
}
