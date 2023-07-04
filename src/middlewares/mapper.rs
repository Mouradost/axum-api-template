use axum::{
    http::{Method, Uri, Request},
    response::{IntoResponse, Response},
    Json,
};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{ctx::Ctx, error::ClientError, Error, Result};

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    // uuid string formatted
    uuid: String,
    // (iso8601 timestamp)
    #[serde(with = "ts_milliseconds")]
    timestamp: DateTime<Utc>,
    // User amd context attribute
    user_id: Option<i64>,
    // Http request attribute
    req_path: String,
    req_method: String,
    // Error attribute
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = Utc::now();
    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));
    // Create the log request line
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp,
        req_path: uri.to_string(),
        req_method: req_method.to_string(),
        user_id: ctx.map(|c| c.user_id()),
        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };
    // Logging
    tracing::info!("{:^12} -\n{:#?}", "LOG_LINE",  json!(log_line));
    // Return
    Ok(())
}

// Response mapper
pub async fn response(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    tracing::debug!("{:^12} - {:^12} - {:^12}", "MIDDLEWARE", "MAPPER", "RESPONSE");

    let uuid = Uuid::new_v4();
    // Get the eventual response error
    let service_error = res.extensions().get::<Error>();
    let client_statut_error = service_error.map(|se| se.client_status_and_error());
    // If client error, build a new response
    let error_response = client_statut_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
            }
            });
            tracing::error!("{:^12} - {:^12} - {:^12} - {:^12} -\n{:#?}", "MIDDLEWARE", "MAPPER", "RESPONSE", "CLIENT_ERROR_BODY", client_error_body);
            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line
    let client_error = client_statut_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error)
        .await
        .expect("Log Line failed !");
    // Return response or the error reponse
    error_response.unwrap_or(res)
}

pub async fn request<B: std::fmt::Debug>(req: Request<B>) -> Request<B> {
    tracing::debug!("{:^12} - {:^12} - {:^12} -\n{:#?}", "MIDDLEWARE", "MAPPER", "REQUEST", req);
    req
}
