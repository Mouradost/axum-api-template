// #![allow(unused)]
use axum::extract::FromRef;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;

mod controler;
mod ctx;
mod error;
mod log;
mod middlewares;
mod model;
mod router;
mod utils;
pub mod args;

pub use self::error::{Error, Result};

pub const AUTH_TOKEN: &str = "auth_token";

#[derive(Clone, FromRef)]
pub struct AppState {
    pool: Pool<Sqlite>,
    jwt_secret: utils::jwt::TokenSecret,
}

pub async fn run(port: u16, log_level: tracing::Level, logger_output: log::LoggerOutput, max_connections: u32) -> Result<()> {
    // Read from .env file
    dotenv().ok();
    let jwt_secret = utils::jwt::TokenSecret(dotenv!("JWT_SECRET"));
    let durl = dotenv!("DATABASE_URL").to_owned();
    // Initialize the database
    let pool = utils::db::connect_db(durl, max_connections).await?;
    // Setup the shared app_state
    let app_state = AppState { pool, jwt_secret };
    // Router
    let routes = router::create_routes(app_state, log_level, logger_output);
    // Get an address
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on {addr}");
    // Bind to the server
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
    Ok(())
}
