use axum::{middleware, routing::post, Router};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::{log, middlewares, AppState};

mod user;

pub fn create_routes(app_state: AppState, log_level: tracing::Level, logger_output: log::LoggerOutput) -> Router {
    // routes
    Router::new()
        .route("/api/logout", post(user::logout))
        .route_layer(middleware::from_fn(middlewares::guard::auth))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            middlewares::resolver::ctx,
        ))
        .route("/api/login", post(user::login))
        .route("/api/register", post(user::register))
        .layer(middleware::map_response(middlewares::mapper::response))
        .layer(CookieManagerLayer::new())
        .fallback_service(ServeDir::new("./"))
        .layer(log::setup_logger(
            logger_output,
            log_level,
        ))
        .layer(middleware::map_request(middlewares::mapper::request))
        .with_state(app_state)
}
