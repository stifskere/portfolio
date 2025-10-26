#![cfg_attr(target_arch = "wasm32", allow(unused))]

use std::io::Error as IoError;

use actix_cors::Cors;
use actix_web::http::header::AUTHORIZATION;
use actix_web::web::Data;
use actix_web::{main, App, HttpServer};
use dotenvy::dotenv;
use thiserror::Error;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt, EnvFilter};

use crate::routes::settings::settings_scope;
use crate::routes::github::github_scope;
use crate::utils::application::context::{AppContext, AppContextError};
use crate::utils::database::settings::setup_settings;

mod models;
mod routes;
mod utils;


/// Holds any error that may happen during the application
/// initialization.
///
/// NOTE: This may not be pretty printed.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error starting the HTTP server, {0:#}")]
    Server(#[from] IoError),

    #[error("Error loading the application context, {0:#}")]
    Context(#[from] AppContextError)
}

#[main]
async fn main() -> Result<(), AppError> {
    // Load the runtime environment variables.
    dotenv().ok();

    // Create and register a logger.
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Setup the dynamic settings trough environment variables.
    setup_settings()
        .await?;

    // Create an app context that connects to
    // other services, holds aplication settings...
    let app_context = Data::new(AppContext::load().await?);

    // Instantiate a new HTTP server.
    HttpServer::new(move || {
        App::new()
            .app_data(app_context.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("https://memw.es")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        AUTHORIZATION,
                    ])
                    .max_age(3600)
            )
            .wrap(TracingLogger::default())
            .service(settings_scope())
            .service(github_scope())
    })
        .bind("0.0.0.0:8081")?
        .run()
        .await?;

    Ok(())
}
