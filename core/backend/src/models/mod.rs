use thiserror::Error;
use sqlx::Error as SqlxError;

use crate::utils::database::connection::Error as ConnectionError;

pub mod reviews;
pub mod variables;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Error connecting to the database: {0:#}")]
    Connection(#[from] ConnectionError),

    #[error("Error querying the database: {0:#}")]
    Query(#[from] SqlxError),

    #[error("Error occurred: {0:#}")]
    Other(String)
}

pub type ModelResult<T> = Result<T, ModelError>;
