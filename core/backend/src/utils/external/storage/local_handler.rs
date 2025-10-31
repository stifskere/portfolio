use async_trait::async_trait;

use crate::utils::external::storage::interface::{GenericError, StorageInterface};

pub struct LocalHandler {
    base_path: String
}

impl LocalHandler {
    pub(super) async fn connect(url: &str) -> Result<Box<dyn StorageInterface>, GenericError> {
        Ok(Box::new(Self {
            base_path: url.to_string()
        }))
    }
}

#[async_trait]
impl StorageInterface for LocalHandler {
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
