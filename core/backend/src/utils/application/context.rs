use std::sync::Arc;

use sqlx::PgPool;
use octocrab::{instance as octocrab_instance, Octocrab};
use thiserror::Error;

use crate::utils::database::connection::{database_connection, DatabaseConnectionError};
use crate::utils::application::environment::{EnvironmentValidationError, PortfolioEnvironment};


/// Holds any error that may happen during an interaction with
/// the application.
#[derive(Error, Debug)]
pub enum AppContextError {
    #[error("Error while loading an environment variable, {0:#}")]
    Envconfig(#[from] EnvironmentValidationError),

    #[error("Error while trying to connect to the database, {0:#}")]
    DatabaseConnection(#[from] DatabaseConnectionError)
}

/// This struct holds data that should be
/// available application wide.
#[derive(Debug)]
pub struct AppContext {
    database_connection: Arc<PgPool>,
    environment: PortfolioEnvironment,
    octocrab: Arc<Octocrab>
}

impl AppContext {
    /// Loads the application context.
    ///
    /// This function should be called on main and
    /// registered as `Data`.
    pub async fn load() -> Result<Self, AppContextError> {
        // pre-load the environment to load context members.
        let environment = PortfolioEnvironment::load_validated()?;

        Ok(Self {
            database_connection: Arc::new(
                database_connection(environment.database_url())
                    .await?
            ),
            octocrab: octocrab_instance(),
            environment: environment,
        })
    }

    /// The environment application relevant environment variables.
    pub fn environment(&self) -> &PortfolioEnvironment {
        &self.environment
    }

    /// An octocrab instance to load raw github data.
    pub fn octocrab(&self) -> &Octocrab {
        &self.octocrab
    }

    /// A postgres database connetion pool.
    pub fn database_pool(&self) -> &PgPool {
        &self.database_connection
    }
}
