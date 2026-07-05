use std::sync::Arc;

use axum::{Router, http::StatusCode, routing::get};

use crate::App;

#[utoipa::path(
    get,
    path = "/api/ping",
    tag = "General",
    summary = "Checks whether the API is alive.",
    responses(
        (status = 200, description = "The API is online."),
    ),
)]
async fn ping() -> StatusCode {
    StatusCode::OK
}

pub(super) fn router() -> Router<Arc<App>> {
    Router::new().route("/api/ping", get(ping))
}
