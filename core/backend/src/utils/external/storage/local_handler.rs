use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{Read, Error as IoError};

use tokio::fs::File;
use async_trait::async_trait;
use thiserror::Error;

use crate::utils::external::storage::interface::{GenericError, StorageInterface};

// TODO: move this to tokio::fs
// TODO: write the methods in the implementation returning LocalHandlerError and polyfill the
// StorageHandler implementation.

#[derive(Error, Debug)]
enum LocalHandlerError {
    #[error(r#"The provided "{base}" path traverses the base path "{objective}"."#)]
    PathTraversal {
        base: String,
        objective: String
    },

    #[error("{0:#}")]
    Io(#[from] IoError)
}

pub struct LocalHandler {
    base_path: PathBuf
}

impl LocalHandler {
    pub(super) async fn connect(url: &str) -> Result<Box<dyn StorageInterface>, GenericError> {
        Ok(Box::new(Self {
            base_path: Path::new(url)
                .canonicalize()?
        }))
    }

    fn check_traversal(&self, path: &Path) -> Result<(), LocalHandlerError> {
        path
            .canonicalize()?
            .starts_with(&self.base_path)
            .then_some(())
            .ok_or_else(|| LocalHandlerError::PathTraversal {
                base: self.base_path
                    .to_string_lossy()
                    .to_string(),
                objective: path
                    .to_string_lossy()
                    .to_string()
            })
    }
}

#[async_trait]
impl StorageInterface for LocalHandler {
    async fn get(&self, path: &str) -> Result<Option<Vec<u8>>, GenericError> {
        let path = Path::new(path);

        if !path.exists() {
            return Ok(None);
        }

        self.check_traversal(path)?;

        let mut file_content = Vec::new();
        let _ = File::open(self.base_path.join(path))?
            .read_to_end(&mut file_content)?;

        Ok(Some(file_content))
    }

    async fn put(&self, path: &str, content: Vec<u8>) -> Result<(), GenericError> {
        todo!()
    }

    async fn delete(&self, path: &str) -> Result<(), GenericError> {
        todo!()
    }
}
