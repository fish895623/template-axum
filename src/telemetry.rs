use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::LogFormat;

/// Initialise the global tracing subscriber.
///
/// - `RUST_LOG` controls filtering (default: `info,tower_http=debug`).
/// - `log_format` selects structured JSON or human-readable output.
pub fn init(log_format: LogFormat) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));

    let registry = tracing_subscriber::registry().with(filter);

    match log_format {
        LogFormat::Json => registry.with(fmt::layer().json()).init(),
        LogFormat::Text => registry.with(fmt::layer()).init(),
    }
}
