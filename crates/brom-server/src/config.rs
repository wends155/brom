use axum::http::HeaderValue;
use std::env;

/// Configuration for the brom server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// List of allowed CORS origins.
    pub cors_origins: Vec<HeaderValue>,
}

impl ServerConfig {
    /// Loads server configuration from environment variables.
    ///
    /// # Panics
    ///
    /// Panics if `BROM_CORS_ORIGINS` is missing or contains invalid header values,
    /// enforcing a fail-fast startup as per coding-standard.md §4.10.
    #[allow(clippy::expect_used)]
    pub fn load_from_env() -> Self {
        // Load .env file if present
        dotenvy::dotenv().ok();

        let origins_str = env::var("BROM_CORS_ORIGINS")
            .expect("BROM_CORS_ORIGINS environment variable is required");

        let cors_origins = origins_str
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<HeaderValue>()
                    .expect("Invalid CORS origin in BROM_CORS_ORIGINS")
            })
            .collect();

        Self { cors_origins }
    }
}
