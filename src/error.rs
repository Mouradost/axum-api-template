use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // Login errors
    #[error("User failed to login")]
    LoginFail,
    #[error("User didn't provide all credentiels")]
    LoginFailMissingCreditienal,
    // Auth errors
    #[error("Authentification missing from cookies")]
    AuthFailNoAuthTokenCookie,
    #[error("JWT wrong format")]
    AuthFailTokenWrongFormat,
    #[error("Context missing from the request")]
    AuthFailCtxNotInRequestExt,
    #[error("JWT_SECRET missing from .env file")]
    SecretJwtEnvUrlNotSet,
    // Database Errors
    #[error("DATABASE_URL missing from .env file")]
    DatabaseEnvUrlNotSet,
    #[error("Could not connect to the database")]
    DatabaseConnectionFailed,
    #[error("Database internal error")]
    DatabaseInternalError,
    // Model Errors
    // User
    #[error("User not in the database")]
    UserNotFound,
    #[error("User already exists in the database")]
    UserExist,
    #[error("User with id: {id}, Not found in the database ")]
    UserDeleteFailedIdNotFound { id: i64 },
    // Fallback
    #[error("Internal server error")]
    InternalServerError
}


// Axum required
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::debug!("{:<12} - {self:?}", "INTO_RES");
        // Create a placeholder Axum response
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        // Inserte the error into response
        response.extensions_mut().insert(self);
        // Return
        response
    }
}

// Custom implementation
impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_EMPY_CREDENTIEL),
            // Auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            Self::UserExist => (StatusCode::BAD_REQUEST, ClientError::USER_EXIST),
            // Fallback
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

// Client Error (Only what we need to expose)
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    // Auth
    LOGIN_FAIL,
    LOGIN_EMPY_CREDENTIEL,
    USER_EXIST,
    NO_AUTH,
    INVALID_PARAMS,
    // Internal error
    SERVICE_ERROR,
}
