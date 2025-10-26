use std::sync::Arc;

use actix_failwrap::{proof_route, ErrorResponse};
use actix_web::{HttpResponse, Scope};
use actix_web::web::{scope, Data};
use octocrab::{Error as OctocrabError, Octocrab};
use thiserror::Error;

use crate::utils::application::context::AppContext;
use crate::utils::requests::github::{fetch_repositories, GithubRequestError};
use crate::models::setting::SettingModelError;
use crate::utils::requests::error_transformer::json_transformer;
use crate::models::setting::Setting;

#[derive(ErrorResponse, Error, Debug)]
#[transform_response(json_transformer)]
enum GithubRouteError {
    #[error("Error while retrieving settings, {0:#}")]
    Setting(#[from] SettingModelError),

    #[error("The username is not configured, GitHub cannot be queried.")]
    #[status_code(424)]
    MissingUsername,

    #[error("The cache was poisoned by someone else.")]
    PoisonedCache,

    #[error("Error while querying the GitHub API: {0:#}")]
    #[status_code(503)]
    GithubRequest(OctocrabError),
}

impl From<GithubRequestError> for GithubRouteError {
    fn from(value: GithubRequestError) -> Self {
        match value {
            GithubRequestError::PoisonedCache => Self::PoisonedCache,
            GithubRequestError::GithubRequest(error) => Self::GithubRequest(error)
        }
    }
}

pub fn github_scope() -> Scope {
    scope("/github")
        .service(get_repositories)
}

#[proof_route("GET /repositories")]
async fn get_repositories(
    context: Data<AppContext>,
    octocrab: Data<Arc<Octocrab>>
) -> Result<HttpResponse, GithubRouteError> {
    let pool = context.database_pool();

    let github_username = Setting::fetch::<String>(
        &pool,
        "GITHUB_USERNAME"
    )
        .await?
        .ok_or(GithubRouteError::MissingUsername)?;

    let repositories = fetch_repositories(&octocrab.into_inner(), &github_username)
        .await?;

    Ok(
        HttpResponse::Ok()
            .json(repositories)
    )
}


