use axum::{http::Request, middleware::Next, response::Response};

use crate::{ctx::Ctx, Result};

pub async fn auth<B>(ctx: Result<Ctx>, req: Request<B>, next: Next<B>) -> Result<Response> {
    tracing::debug!(
        "- {:<12} - {:<12} - {:<12} - {:#?}",
        "MIDDLEWARE",
        "GUARD",
        "AUTH",
        ctx
    );

    ctx?;

    Ok(next.run(req).await)
}
