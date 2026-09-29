use std::net::{IpAddr, SocketAddr};

use clap::{Parser, ValueEnum};
use validator::Validate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LogFormat {
    Text,
    Json,
}

/// Application configuration, read from CLI flags or environment variables.
///
/// Flags take precedence over environment variables, which take precedence over defaults.
/// Parse errors are reported by clap; value rules are checked by [`Config::load`].
#[derive(Debug, Clone, Parser, Validate)]
#[command(version, about)]
pub struct Config {
    /// Address to bind to.
    #[arg(long, env = "HOST", default_value = "0.0.0.0")]
    pub host: IpAddr,

    /// Port to listen on.
    #[arg(long, env = "PORT", default_value_t = 3000)]
    #[validate(range(min = 1))]
    pub port: u16,

    /// Log output format.
    #[arg(long, env = "LOG_FORMAT", value_enum, default_value_t = LogFormat::Text)]
    pub log_format: LogFormat,
}

impl Config {
    /// Load `.env` (if present), parse flags and environment, then validate.
    ///
    /// Exits with a usage message on parse errors; returns an error on validation failures.
    pub fn load() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        let config = Self::parse();
        config.validate()?;
        Ok(config)
    }

    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<Config, clap::Error> {
        Config::try_parse_from(std::iter::once("template-axum").chain(args.iter().copied()))
    }

    #[test]
    fn defaults() {
        let config = parse(&["--host", "127.0.0.1"]).unwrap();
        assert_eq!(config.port, 3000);
        assert_eq!(config.log_format, LogFormat::Text);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn rejects_invalid_host() {
        assert!(parse(&["--host", "not-an-ip"]).is_err());
    }

    #[test]
    fn rejects_out_of_range_port() {
        assert!(parse(&["--port", "70000"]).is_err());
    }

    #[test]
    fn rejects_port_zero() {
        let config = parse(&["--port", "0"]).unwrap();
        assert!(config.validate().is_err());
    }

    #[test]
    fn rejects_unknown_log_format() {
        assert!(parse(&["--log-format", "xml"]).is_err());
    }
}
