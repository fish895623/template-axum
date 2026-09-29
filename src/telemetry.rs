use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialise the global tracing subscriber.
///
/// - `RUST_LOG` controls filtering (default: `info,tower_http=debug`).
/// - `LOG_FORMAT=json` switches to structured JSON output; anything else is human-readable.
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));

    let registry = tracing_subscriber::registry().with(filter);

    match std::env::var("LOG_FORMAT").as_deref() {
        Ok("json") => registry.with(fmt::layer().json()).init(),
        _ => registry.with(fmt::layer()).init(),
    }
}
