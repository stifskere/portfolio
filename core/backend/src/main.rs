use std::io::Error as IoError;

use actix_cors::Cors;
use actix_web::http::header::AUTHORIZATION;
use actix_web::web::Data;
use actix_web::{main, App, HttpServer};
use dotenvy::dotenv;
use octocrab::instance as octocrab_instance;
use thiserror::Error;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt, EnvFilter};

use crate::models::ModelError;
use crate::routes::settings::settings_scope;
use crate::routes::github::github_scope;
use crate::utils::database::settings::setup_settings;

mod models;
mod routes;
mod utils;


#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error starting the HTTP server: {0:#}")]
    Server(#[from] IoError),

    #[error("Error while setting up a database state: {0:#}")]
    Database(#[from] ModelError)
}

#[main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    setup_settings()
        .await?;

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("https://memw.es")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                AUTHORIZATION,
            ])
            .max_age(3600);

        let octocrab_instance  = Data::new(octocrab_instance());

        App::new()
            .app_data(octocrab_instance.clone())
            .wrap(cors)
            .wrap(TracingLogger::default())
            .service(settings_scope())
            .service(github_scope())
    })
        .bind("0.0.0.0:8081")?
        .run()
        .await?;

    Ok(())
}
