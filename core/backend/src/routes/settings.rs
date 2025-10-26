use actix_failwrap::{proof_route, ErrorResponse};
use actix_web::{HttpResponse, Scope};
use actix_web::web::{scope, Bytes, Data, Path, Query};
use serde::Deserialize;
use thiserror::Error;

use crate::models::setting::{Setting, SettingModelError};
use crate::utils::application::context::AppContext;
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

    #[error("Error while retrieving settings, {0:#}")]
    Model(#[from] SettingModelError)
}

pub fn settings_scope() -> Scope {
    scope("/setting")
        .service(get_setting)
        .service(post_setting)
}

#[derive(Deserialize)]
struct SettingPathParameters {
    key: String
}

#[derive(Deserialize)]
struct PostSettingQueryParameters {
    #[serde(default)]
    replace: bool,
    #[serde(default)]
    require_auth: bool
}

#[proof_route("GET /{key}")]
async fn get_setting(
    context: Data<AppContext>,
    path: Path<SettingPathParameters>,
    auth: OptionalAuth
) -> Result<HttpResponse, SettingRouteError> {
    let pool = context.database_pool();

    let setting = Setting::fetch::<Vec<u8>>(&pool, &path.key)
        .await?
        .ok_or(SettingRouteError::NotConfigured)?;

    if setting.requires_auth() && !auth.is_authorized() {
        return Err(SettingRouteError::NotAuthenticated);
    }

    Ok(
        HttpResponse::Ok()
            .body(setting.into_value())
    )
}

#[proof_route("POST /{key}")]
async fn post_setting(
    context: Data<AppContext>,
    path: Path<SettingPathParameters>,
    query: Query<PostSettingQueryParameters>,
    auth: OptionalAuth,
    body: Bytes
) -> Result<HttpResponse, SettingRouteError> {
    if !auth.is_authorized() {
        return Err(SettingRouteError::NotAuthenticated);
    }

    let pool = context.database_pool();

    if query.replace {
        Setting::store_or_update(
            &pool,
            query.require_auth,
            &path.key,
            &body.to_vec()
        )
            .await?;
    } else {
        Setting::store_or_ignore(
            &pool,
            query.require_auth,
            &path.key,
            &body.to_vec()
        )
            .await?;
    };

    Ok(
        HttpResponse::Created()
            .finish()
    )
}
