use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts};
use crate::{Result, Error};

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: i64,
}

impl Ctx {
    // Constructor
    pub fn new(user_id: i64) -> Self {
        Self { user_id }
    }
    // Property Accessors
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

// Ctx Extractor
#[async_trait] // Maybe in the future rust will have it
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        tracing::debug!("- {:^12} - {:^12}", "EXTRACTOR", "CTX");
        // Inject the context
        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}
