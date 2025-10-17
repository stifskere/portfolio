use std::sync::Arc;

use actix_failwrap::{proof_route, ErrorResponse};
use actix_web::{web::{scope, Data}, HttpResponse, Scope};
use octocrab::{Error as OctocrabError, Octocrab};
use thiserror::Error;

use crate::utils::requests::github::{fetch_repositories, GithubRequestError};
use crate::utils::requests::error_transformer::json_transformer;
use crate::models::ModelError;
use crate::models::variables::Variable;

#[derive(ErrorResponse, Error, Debug)]
#[transform_response(json_transformer)]
enum GithubRouteError {
    #[error("Error while querying the database: {0:#}")]
    Model(#[from] ModelError),

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
async fn get_repositories(octocrab: Data<Arc<Octocrab>>) -> Result<HttpResponse, GithubRouteError> {
    let github_username = Variable::fetch::<String>("GITHUB_USERNAME")
        .await?
        .ok_or(GithubRouteError::MissingUsername)?;

    let repositories = fetch_repositories(&octocrab.into_inner(), &github_username)
        .await?;

    Ok(
        HttpResponse::Ok()
            .json(repositories)
    )
}


