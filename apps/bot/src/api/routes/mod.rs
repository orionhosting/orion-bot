mod ping;

use std::sync::Arc;

use axum::{Json, Router, routing::get};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipauto::utoipauto;

use crate::App;

/// Build the api router.
pub(super) fn all_routes() -> Router<Arc<App>> {
    Router::new().merge(router()).merge(ping::router())
}

#[utoipauto(paths = "apps/bot/src/api/routes")]
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Orion Bot API",
        version = env!("CARGO_PKG_VERSION"),
        description = "The Orion Bot API.",
        contact(
            name = "Orion Hosting",
            url = "https://github.com/orionhosting"
        )
    ),
    tags(
        (name = "General", description = "General API endpoints")
    )
)]
pub(super) struct ApiDoc;

// `/` router

#[derive(Serialize, ToSchema)]
struct ApiInfo {
    version: String,
    git_commit: String,
    built_at: u64,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "General",
    summary = "Get the API info.",
    responses(
        (status = 200, description = "The API info.", body = ApiInfo),
    ),
)]
async fn api_info() -> Json<ApiInfo> {
    Json(ApiInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        git_commit: env!("APP_REV").to_string(),
        built_at: env!("BUILD_TIME").parse().unwrap_or(0),
    })
}

pub(super) fn router() -> Router<Arc<App>> {
    Router::new().route("/", get(api_info))
}
