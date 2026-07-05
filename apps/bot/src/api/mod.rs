mod routes;

use std::{net::SocketAddr, sync::Arc};

use axum::{Json, http::StatusCode};
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{App, config::Config};
use routes::ApiDoc;

/// Starts the API.
pub async fn start_api(app: Arc<App>) {
    // ratelimiter
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(5)
            .burst_size(100)
            .use_headers()
            .finish()
            .unwrap(),
    );

    let limiter = governor_conf.limiter().clone();
    tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(60);
        loop {
            tokio::time::sleep(interval).await;
            limiter.retain_recent();
        }
    });

    let router = routes::all_routes()
        .merge(SwaggerUi::new("/docs").url("/api/docs/openapi.json", ApiDoc::openapi()))
        .fallback(not_found)
        .layer(GovernorLayer::new(governor_conf))
        .with_state(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], Config::get().port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("api listening on {addr}");
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "message": "Not found" })),
    )
}
