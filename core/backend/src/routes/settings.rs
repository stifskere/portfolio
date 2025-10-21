use std::string::FromUtf8Error;

use actix_failwrap::{proof_route, ErrorResponse};
use actix_web::{HttpResponse, Scope};
use actix_web::web::{scope, Bytes};
use thiserror::Error;

use crate::models::setting::Setting;
use crate::models::ModelError;
use crate::utils::requests::authentication::OptionalAuth;
use crate::utils::requests::error_transformer::json_transformer;

#[derive(ErrorResponse, Error, Debug)]
#[transform_response(json_transformer)]
enum SettingRouteError {
    #[error("Method Not Allowed.")]
    #[status_code(405)]
    NotAuthenticated,

    #[error("The requested variable is not configured.")]
    #[status_code(404)]
    NotConfigured,

    #[error("Error while querying the database: {0:#}")]
    Model(#[from] ModelError),

    #[error("The value sent is invalid.")]
    #[status_code(400)]
    InvalidString(#[from] FromUtf8Error)
}

pub fn settings_scope() -> Scope {
    scope("/setting")
        .service(get_presentation)
        .service(set_presentation)
        .service(get_moto)
        .service(set_moto)
}

#[proof_route("GET /presentation")]
async fn get_presentation() -> Result<HttpResponse, SettingRouteError> {
    let presentation = Setting::fetch::<String>("PORTFOLIO_PRESENTATION")
        .await?
        .ok_or(SettingRouteError::NotConfigured)?;

    Ok(
        HttpResponse::Ok()
            .body(presentation)
    )
}

#[proof_route("PUT /presentation")]
async fn set_presentation(auth: OptionalAuth, value: Bytes) -> Result<HttpResponse, SettingRouteError> {
    if !auth.is_authorized() {
        return Err(SettingRouteError::NotAuthenticated);
    }

    Setting::store_or_update(
        "PORTFOLIO_PRESENTATION",
        String::from_utf8(value.to_vec())?
    )
        .await?;

    Ok(
        HttpResponse::NoContent()
            .finish()
    )
}

#[proof_route("GET /moto")]
async fn get_moto() -> Result<HttpResponse, SettingRouteError> {
    let moto = Setting::fetch::<String>("PORTFOLIO_MOTO")
        .await?
        .ok_or(SettingRouteError::NotConfigured)?;

    Ok(
        HttpResponse::Ok()
            .body(moto)
    )
}

#[proof_route("PUT /moto")]
async fn set_moto(auth: OptionalAuth, value: Bytes) -> Result<HttpResponse, SettingRouteError> {
    if !auth.is_authorized() {
        return Err(SettingRouteError::NotAuthenticated);
    }

    Setting::store_or_update(
        "PORTFOLIO_MOTO",
        String::from_utf8(value.to_vec())?
    )
        .await?;

    Ok(
        HttpResponse::NoContent()
            .finish()
    )
}

