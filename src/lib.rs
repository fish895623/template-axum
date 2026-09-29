pub mod routes;
pub mod telemetry;

use axum::Router;
use tower_http::trace::TraceLayer;

/// Build the application router with all routes and middleware.
pub fn app() -> Router {
    Router::new()
        .merge(routes::router())
        .layer(TraceLayer::new_for_http())
}
