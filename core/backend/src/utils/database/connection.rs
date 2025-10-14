use std::{env::VarError, sync::OnceLock};
use std::env::var;

use sqlx::{postgres::PgPoolOptions, PgPool, Error as SqlxError};
use thiserror::Error;

#[macro_export]
macro_rules! db {
    () => {
        $crate::utils::database::connection::get_connection().await
    };
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Couldn't find DATABASE_URL environment variable.")]
    MissingEnvironment(#[from] VarError),

    #[error("Couldn't perform connection to the database.")]
    Connection(#[from] SqlxError)
}

static CONNECTION: OnceLock<PgPool> = OnceLock::new();

pub async fn get_connection() -> Result<&'static PgPool, Error> {
    if let Some(connection) = CONNECTION.get() {
        return Ok(connection);
    }

    let connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&var("DATABASE_URL")?)
        .await?;

    Ok(CONNECTION.get_or_init(|| connection))
}
