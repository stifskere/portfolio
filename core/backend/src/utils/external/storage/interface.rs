use std::error::Error as StdError;

use thiserror::Error;
use async_trait::async_trait;

use crate::utils::{application::environment::PortfolioEnvironment, external::storage::{local_handler::LocalHandler, s3_handler::S3Handler}};

/// ;(
pub(super) type GenericError = Box<dyn StdError + Send + Sync + 'static>;

/// This trait is commonly implemented by storage interface
/// structures. This is to be publicly exposed in an instance
/// url resolver.
#[async_trait]
pub trait StorageInterface: Send + Sync {
    /// This method will obtain a file from the connected storage
    /// interface.
    ///
    /// Since `NotFound` errors are not advised, this returns
    /// `Ok(None)` in the case a file does not exist.
    async fn get(&self, file_path: &str) -> Result<Option<Vec<u8>>, GenericError>;

    /// This method will create or update a file from the connected
    /// storage interface.
    async fn put(&self, file_path: &str, content: Vec<u8>) -> Result<(), GenericError>;

    /// This method will delete a file from the connected storage interface
    /// no feedback should be returned, if it wasn't deleted because
    /// it didn't exist in the first place that's indifferent.
    async fn delete(&self, file_path: &str) -> Result<(), GenericError>;
}

/// Errors happening while connecting to the user configured
/// storage handler.
///
/// Errors within the interface should happen in errors declared
/// in each handler.
#[derive(Error, Debug)]
pub enum StorageConnectionError {
    #[error("Couldn't resolve the protocol, URL must contain `://`.")]
    CannotResolveProtocol,

    #[error("Couldn't connect to a server with protocol '{0:#}', unknown protocol.")]
    UnknownProtocol(String),

    #[error("")]
    ConnectionError(#[from] GenericError)
}

/// Connect to the configured storage interface, the connection
/// url should be provided in the environment singleton loaded
/// in `AppContext`.
pub async fn connect_storage(environment: &PortfolioEnvironment)
    -> Result<Box<dyn StorageInterface>, StorageConnectionError> {
    let (protocol, url) = environment.storage_url()
        .split_once("://")
        .ok_or(StorageConnectionError::CannotResolveProtocol)?;

    Ok(
        match protocol {
            "local" => LocalHandler::connect(url).await?,
            "s3" => S3Handler::connect(url).await?,

            other => {
                return Err(StorageConnectionError::UnknownProtocol(other.to_string()));
            }
        }
    )
}
