use axum::{extract::State, http::Request, middleware::Next, response::Response};
use tower_cookies::{Cookie, Cookies};

use crate::{ctx::Ctx, utils, Error, Result, AUTH_TOKEN};

pub async fn ctx<B>(
    cookies: Cookies,
    State(jwt_secret): State<utils::jwt::TokenSecret>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    tracing::debug!(
        "- {:^12} - {:^12} - {:^12}",
        "MIDDLEWARE",
        "RESOLVER",
        "CTX"
    );
    // Get the token as a String
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    tracing::debug!(
        "- {:^12} - {:^12} - {:^12} - {:#?}",
        "MIDDLEWARE",
        "RESOLVER",
        "CTX",
        auth_token
    );
    // Compute Result<Ctx>
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(|token| utils::jwt::parse_token(token, jwt_secret.0))
    {
        Ok(claims) => Ok(Ctx::new(claims.get_user_id())),
        Err(e) => Err(e),
    };
    tracing::debug!(
        "- {:^12} - {:^12} - {:^12} - {:#?}",
        "MIDDLEWARE",
        "RESOLVER",
        "CTX",
        result_ctx
    );
    // Remove the cookie if there is an error other than AuthFailNoAuthTokenCookie
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    };
    // Store result_ctx in the request extension
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}
