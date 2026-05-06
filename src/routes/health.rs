use axum::{Json, Router, routing::get};
use serde::Serialize;

/// Health check response returned by the API.
#[derive(Debug, Serialize)]
pub(crate) struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

/// Builds the health check routes.
pub(crate) fn router() -> Router {
    Router::new().route("/health", get(health))
}

/// Returns the current service health status.
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "astronavis-api",
    })
}
