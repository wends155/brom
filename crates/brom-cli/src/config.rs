//! Configuration management for the brom CLI.

/// Application configuration derived from environment variables.
pub struct AppConfig {
    /// Path to the `SQLite` database file.
    pub db_path: String,
}

impl AppConfig {
    /// Loads the application configuration from the environment.
    ///
    /// Defaults `DATABASE_URL` to "brom.db" if not specified.
    pub fn load() -> Self {
        dotenvy::dotenv().ok();
        let db_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "brom.db".to_string());
        Self { db_path }
    }
}
