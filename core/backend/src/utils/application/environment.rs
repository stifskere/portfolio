use email_address::EmailAddress;
use envconfig::{Envconfig, Error as EnvconfigError};
use thiserror::Error;

// NOTE: PE stands for portfolio environment.

/// Holds any validation errors that may occur
/// while loading the environment.
///
/// Throwing an error in here means the application
/// will crash.
#[derive(Error, Debug)]
pub enum EnvironmentValidationError {
    #[error("Error while loading an environment variable, {0:#}")]
    Envconfig(#[from] EnvconfigError),

    #[error("PE_ADMIN_EMAIL contains an invalid email address.")]
    InvalidEmail
}

/// This struct holds the relevant
/// environment variables for this application.
#[derive(Envconfig, Debug)]
pub struct PortfolioEnvironment {
    #[envconfig(from = "PE_ADMIN_EMAIL")]
    admin_email: String,
    #[envconfig(from = "PE_ADMIN_PASSWORD")]
    admin_password: String
}

impl PortfolioEnvironment {
    pub fn load_validated() -> Result<Self, EnvironmentValidationError> {
        let instance = Self::init_from_env()?;

        if !EmailAddress::is_valid(instance.admin_email()) {
            log::error!("The PE_ADMIN_EMAIL is not a valid email address.");
            
        }

        Ok(instance)
    }

    /// The email for the admin panel authentication.
    #[inline]
    pub fn admin_email(&self) -> &str {
        &self.admin_email
    }

    /// The unencrypted password for the admin panel authentication.
    ///
    /// Security wise this isn't accessed by anyone nor printed
    /// anywhere, this should be held securely at host level.
    #[inline]
    pub fn admin_password(&self) -> &str {
        &self.admin_password
    }
}
