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

use crate::routes::variables::variables_scope;
use crate::routes::github::github_scope;


mod models;
mod routes;
mod utils;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error starting the HTTP server: {0:#}")]
    Server(#[from] IoError)
}

#[main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

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
            .service(variables_scope())
            .service(github_scope())
//            .service(reviews_scope()) // This may be enabled in a future.
    })
        .bind("0.0.0.0:8081")?
        .run()
        .await?;

    Ok(())
}
