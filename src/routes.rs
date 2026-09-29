use axum::{
    Json, Router,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/echo", post(echo))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Health {
    pub status: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}

#[tracing::instrument]
async fn health() -> Json<Health> {
    Json(Health {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        timestamp: Utc::now(),
    })
}

#[derive(Debug, Deserialize)]
pub struct EchoRequest {
    pub message: String,
    /// Optional client-side timestamp, parsed from RFC 3339.
    pub sent_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoResponse {
    pub message: String,
    pub sent_at: Option<DateTime<Utc>>,
    pub received_at: DateTime<Utc>,
    pub meta: Value,
}

#[tracing::instrument(skip_all)]
async fn echo(Json(req): Json<EchoRequest>) -> Json<EchoResponse> {
    let received_at = Utc::now();
    let latency_ms = req.sent_at.map(|s| (received_at - s).num_milliseconds());
    tracing::info!(?latency_ms, "echo received");

    Json(EchoResponse {
        meta: json!({ "length": req.message.chars().count(), "latency_ms": latency_ms }),
        message: req.message,
        sent_at: req.sent_at,
        received_at,
    })
}
