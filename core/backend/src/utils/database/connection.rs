use std::sync::OnceLock;
use std::env::VarError;
use std::env::var;

use sqlx::{PgPool, Error as SqlxError};
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;

/// Holds any errors related to database connection.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Couldn't find DATABASE_URL environment variable.")]
    MissingEnvironment(#[from] VarError),

    #[error("Couldn't perform connection to the database.")]
    Connection(#[from] SqlxError)
}

/// This function obtains a postgres connection
/// from storage or stores a new one, since this
/// is a small project this behavior is inherited
/// from older projects of mine.
pub async fn connect() -> Result<&'static PgPool, Error> {
    static CONNECTION: OnceLock<PgPool> = OnceLock::new();

    if let Some(connection) = CONNECTION.get() {
        return Ok(connection);
    }

    let connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&var("DATABASE_URL")?)
        .await?;

    Ok(CONNECTION.get_or_init(|| connection))
}
