use axum::{extract::State, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::{Pool, Sqlite};
use tower_cookies::{Cookie, Cookies};

use crate::{
    controler,
    model,
    utils::{self, jwt::Claims},
    Error, Result, AUTH_TOKEN,
};

pub async fn register(
    State(pool): State<Pool<Sqlite>>,
    Json(mut payload): Json<model::user::UserForCreate>,
) -> Result<Json<serde_json::Value>> {
    tracing::debug!("- {:^12} - {:^12} - {:^12} ", "HANDLER", "USER", "REGISTER");
    // Hashing the password before creating the user
    payload.password = hash_password(payload.password)?;
    // Create a user
    let user = controler::user::create_user(pool, payload).await?;
    // Create a response
    Ok(Json(json!(
        {
            "result":
            {
                "success": true
            },
            "user": {
                "id": user.id,
                "username": user.username
            }
        }
    )))
}

pub async fn login(
    cookies: Cookies,
    State(pool): State<Pool<Sqlite>>,
    State(jwt_secret): State<utils::jwt::TokenSecret>,
    Json(payload): Json<model::user::UserForCreate>,
) -> Result<Json<serde_json::Value>> {
    tracing::debug!("- {:^12} - {:^12} - {:^12} ", "HANDLER", "USER", "LOGIN");
    // Check if the creds are empty
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(Error::LoginFailMissingCreditienal);
    }
    // Get user by username
    let user = controler::user::get_user_by_username(pool, payload.username.clone()).await?;
    // Verify the user credentials
    if payload.username != user.username || !verify_password(payload.password, user.password.as_str())? {
        return Err(Error::LoginFailMissingCreditienal);
    }
    // Create a token based on the user info
    let claims = Claims::new(user.id.ok_or(Error::LoginFail)?);
    cookies.add(Cookie::new(
        AUTH_TOKEN,
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.0.as_ref()),
        )
        .map_err(|error| {
            tracing::error!(
                "- {:^12} - {:^12} - {:^12} -\n{:#?}",
                "HANDLER",
                "USER",
                "LOGIN",
                error
            );
            Error::LoginFail
        })?,
    ));
    // Create success body
    Ok(Json(json!({
        "result": {
        "success": true
    }
    })))
}

pub async fn logout(cookies: Cookies) -> Result<Json<serde_json::Value>> {
    cookies.remove(Cookie::named(AUTH_TOKEN));
    tracing::debug!(
        "- {:^12} - {:^12} - {:^12} -\n{:#?}",
        "HANDLER",
        "USER",
        "LOGOUT",
        cookies
    );
    // Create success body
    Ok(Json(json!({
        "result": {
        "success": true
    }
    })))
}

fn hash_password(password: String) -> Result<String> {
    bcrypt::hash(password, 14).map_err(|error| {
        tracing::error!("- {:^12} -\n{:#?}", "HASH_PASSWORD", error);
        Error::InternalServerError
    })
}

fn verify_password(password: String, hash: &str) -> Result<bool> {
    bcrypt::verify(password, hash).map_err(|error| {
        tracing::error!("- {:^12} -\n{:#?}", "VERIFY_PASSWORD", error);
        Error::InternalServerError
    })
}
