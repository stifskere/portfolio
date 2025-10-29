use sqlx::{PgPool, Error as SqlxError};
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;


/// NOTE: This is a single responsability module
/// that may seem redundant, but it's built like this
/// to not change the layout and for future proofing.

/// Holds any errors related to database connection.
#[derive(Error, Debug)]
pub enum DatabaseConnectionError {
    #[error("Couldn't perform connection to the database.")]
    Connection(#[from] SqlxError)
}

/// This function obtains a postgres connection
/// from storage or stores a new one, since this
/// is a small project this behavior is inherited
/// from older projects of mine.
pub async fn database_connection(url: &str) -> Result<PgPool, DatabaseConnectionError> {
    Ok(
        PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?
    )
}
