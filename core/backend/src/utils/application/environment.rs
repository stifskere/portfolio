use envconfig::Envconfig;


/// This struct holds the relevant
/// environment variables for this application.
#[derive(Envconfig, Debug)]
pub struct PortfolioEnvironment {
    #[envconfig(from = "PC_ADMIN_USER")]
    admin_user: String,
    #[envconfig(from = "PC_ADMIN_PASSWORD")]
    admin_password: String
}

impl PortfolioEnvironment {
    /// The username for the admin panel authentication.
    #[inline]
    pub fn admin_user(&self) -> &str {
        &self.admin_user
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
