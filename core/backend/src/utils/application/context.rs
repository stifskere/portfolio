use std::sync::Arc;

// use sqlx::PgPool;
use envconfig::Envconfig;
use octocrab::{instance as octocrab_instance, Octocrab};
use thiserror::Error;

use crate::utils::application::environment::{EnvironmentValidationError, PortfolioEnvironment};


/// Holds any error that may happen during an interaction with
/// the application.
#[derive(Error, Debug)]
pub enum AppContextError {
    #[error("Error while loading an environment variable, {0:#}")]
    Envconfig(#[from] EnvironmentValidationError)
}

/// This struct holds data that should be
/// available application wide.
#[derive(Debug)]
pub struct AppContext {
//    database_connection: Arc<PgPool>,
    environment: PortfolioEnvironment,
    octocrab: Arc<Octocrab>
}

impl AppContext {
    /// Loads the application context.
    ///
    /// This function should be called on main and
    /// registered as `Data`.
    pub fn load() -> Result<Self, AppContextError> {
        Ok(Self {
            environment: PortfolioEnvironment::load_validated()?,
            octocrab: octocrab_instance()
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
}
