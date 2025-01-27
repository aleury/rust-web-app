use crate::ctx::Ctx;
use crate::web;
use crate::Result;
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;
use uuid::Uuid;

pub async fn log_request(
    uuid: Uuid,
    method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    server_error: Option<&web::Error>,
    client_error: Option<web::ClientError>,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let client_error_type = client_error.map(|err| err.as_ref().to_string());
    let error_type = server_error.map(|err| err.as_ref().to_string());
    let error_data = serde_json::to_value(server_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),
        http_path: uri.to_string(),
        http_method: method.to_string(),
        user_id: ctx.map(|c| c.user_id()),
        client_error_type,
        error_type,
        error_data,
    };

    debug!("REQUEST LOG LINE:\n{}", json!(log_line));

    // TODO - Send to cloud-watch.

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      // uuid string formatted
    timestamp: String, // (should be iso8601)

    // -- User and context attributes.
    user_id: Option<u64>,

    // -- http request attributes.
    http_path: String,
    http_method: String,

    // -- Error attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
