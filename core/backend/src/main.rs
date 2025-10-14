use std::io::Error as IoError;

use actix_web::{main, App, HttpServer};
use dotenvy::dotenv;
use thiserror::Error;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt, EnvFilter};


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
        App::new()
            .wrap(TracingLogger::default())
    })
        .bind("0.0.0.0:8081")?
        .run()
        .await?;

    Ok(())
}
