use crate::ctx::Ctx;
use crate::log::log_request;
use crate::web;
use axum::{
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

pub async fn mw_response_map(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    debug!("{:<12} - mw_response_map", "RESPONSE_MAP");
    let uuid = Uuid::new_v4();

    // -- Get the eventual response error.
    let web_error = res.extensions().get::<web::Error>();
    let client_status_error = web_error.map(|err| err.client_status_and_error());

    // -- If client error, build the new response.
    let error_response =
        client_status_error
            .as_ref()
            .map(|(status_code, client_error)| {
                let client_error_body = json!({
                    "error": {
                        "type": client_error.as_ref(),
                        "req_uuid": uuid.to_string(),
                    }
                });

                debug!("CLIENT ERROR BODY:\n{client_error_body}");

                // Build the new response from the client_error_body
                (*status_code, Json(client_error_body)).into_response()
            });

    // Build and log the server log line
    let client_error = client_status_error.unzip().1;
    // TODO: Need to handle if log_request fail (but should not fail request)
    let _ = log_request(uuid, req_method, uri, ctx, web_error, client_error).await;

    debug!("\n");

    error_response.unwrap_or(res)
}
