use actix_failwrap::{proof_route, ErrorResponse};
use actix_web::{HttpResponse, Scope};
use actix_web::web::{scope, Json, Path};
use thiserror::Error;

use crate::models::ModelError;
use crate::models::reviews::{PartialReview, Review};
use crate::utils::requests::authentication::OptionalAuth;

#[derive(ErrorResponse, Error, Debug)]
enum ReviewRouteError {
    #[error("Error while querying the database: {0:#}")]
    Model(#[from] ModelError),

    #[error("A review with the ID {0:?} was not found.")]
    #[status_code(404)]
    ReviewNotFound(String),

    #[error("Not found.")]
    #[status_code(404)]
    GeneralNotFound,

    #[error("The provided ID {0:?} is an invalid V4 UUID.")]
    InvalidId(String),
}

#[expect(unused)]
pub fn reviews_scope() -> Scope {
    scope("/reviews")
        .service(get_all_reviews)
        .service(post_review_request)
        .service(put_edit_review)
        .service(post_review_set_editable)
        .service(delete_review)
}

async fn get_review(id: String) -> Result<Review, ReviewRouteError> {
    Review::get_review(
        id
            .clone()
            .parse()
            .map_err(|_| ReviewRouteError::InvalidId(id.clone()))?
    )
        .await?
        .ok_or(ReviewRouteError::ReviewNotFound(id))
}

#[proof_route("GET /")]
async fn get_all_reviews() -> Result<HttpResponse, ReviewRouteError> {
    let reviews = Review::get_all_reviews()
        .await?;

    Ok(
        HttpResponse::Ok()
            .json(reviews)
    )
}

#[proof_route("POST /")]
async fn post_review_request() -> Result<HttpResponse, ReviewRouteError> {
    let request = Review::new_request()
        .await?;

    Ok(
        HttpResponse::Ok()
            .json(request)
    )
}

#[proof_route("PUT /{id}/")]
async fn put_edit_review(
    id: Path<String>,
    auth: OptionalAuth,
    partial: Json<PartialReview>
) -> Result<HttpResponse, ReviewRouteError> {
    let id = id.into_inner();

    let mut review = get_review(id.clone()).await?;

    if !review.can_edit() && !auth.is_authorized() {
        return Err(ReviewRouteError::ReviewNotFound(id));
    }

    review.submit_review(partial.into_inner())
        .await?;

    Ok(
        HttpResponse::NoContent()
            .finish()
    )
}

#[proof_route("POST /{id}/editable")]
async fn post_review_set_editable(id: Path<String>, auth: OptionalAuth) -> Result<HttpResponse, ReviewRouteError> {
    if !auth.is_authorized() {
        return Err(ReviewRouteError::GeneralNotFound);
    }

    let mut review = get_review(id.into_inner()).await?;
    review.set_editable().await?;

    Ok(
        HttpResponse::NoContent()
            .finish()
    )
}

#[proof_route("DELETE /{id}")]
async fn delete_review(id: Path<String>, auth: OptionalAuth) -> Result<HttpResponse, ReviewRouteError> {
    if !auth.is_authorized() {
        return Err(ReviewRouteError::GeneralNotFound);
    }

    let review = get_review(id.into_inner()).await?;
    review.delete_review().await?;

    Ok(
        HttpResponse::NoContent()
            .finish()
    )
}
