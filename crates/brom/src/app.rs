//! High-level application builder for brom CMS.
//!
//! `BromApp` encapsulates the startup ceremony into a fluent API,
//! enabling a complete CMS server in ~5 method calls.

use brom_core::entity::{EntitySchema, SchemaInfo};
use brom_core::schema::SchemaRegistry;
use brom_db::{DbPool, MigrationRunner};
use brom_server::AppState;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;

/// High-level application builder for brom CMS.
///
/// Encapsulates environment loading, tracing initialization, database pooling,
/// migrations, schema registration, and server binding into a fluent API.
///
/// # Examples
///
/// ```rust,no_run
/// use brom::BromApp;
/// // Assuming Post and Category implement EntitySchema via #[derive(BromEntity)]
/// // BromApp::new()
/// //     .entity::<Post>()
/// //     .entity::<Category>()
/// //     .serve("0.0.0.0:3000")
/// //     .await
/// //     .expect("server failed");
/// ```
pub struct BromApp {
    pub(crate) db_path: Option<String>,
    pub(crate) schemas: Vec<SchemaInfo>,
    pub(crate) cors_origins: Vec<String>,
    pub(crate) migrations_dir: Option<String>,
}

impl BromApp {
    /// Creates a new `BromApp` builder with default configuration.
    ///
    /// Loads environment variables from a `.env` file if present, and sets up
    /// structured logging under `tracing` via the `RUST_LOG` environment filter.
    ///
    /// # Returns
    ///
    /// A default `BromApp` builder instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brom::BromApp;
    ///
    /// let app = BromApp::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        // Load .env file if present — ignore errors (file may not exist).
        dotenvy::dotenv().ok();

        // Initialize tracing subscriber with env-filter support.
        // Ignore errors if a subscriber is already set (e.g., in tests).
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
            )
            .try_init();

        Self {
            db_path: None,
            schemas: Vec::new(),
            cors_origins: Vec::new(),
            migrations_dir: None,
        }
    }

    /// Overrides the database file path.
    ///
    /// By default, the application resolves the database path using the `DATABASE_URL`
    /// environment variable, falling back to `"brom.db"` if unset.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to the target `SQLite` database file.
    ///
    /// # Returns
    ///
    /// The updated builder instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brom::BromApp;
    ///
    /// let app = BromApp::new().database("my_custom_db.sqlite");
    /// ```
    #[must_use]
    pub fn database(mut self, path: &str) -> Self {
        self.db_path = Some(path.to_string());
        self
    }

    /// Registers an entity type with the CMS schema registry.
    ///
    /// Registers the entity's table structure and field metadata. Registered entities
    /// are exposed as REST API routes and automatically migrated.
    ///
    /// # Returns
    ///
    /// The updated builder instance.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use brom::{BromApp, BromEntity};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize, BromEntity)]
    /// struct Post {
    ///     pub title: String,
    /// }
    ///
    /// let app = BromApp::new().entity::<Post>();
    /// ```
    #[must_use]
    pub fn entity<T: EntitySchema>(mut self) -> Self {
        self.schemas.push(T::schema_info());
        self
    }

    /// Overrides the CORS allowed origins.
    ///
    /// By default, allowed origins are read from the `BROM_CORS_ORIGINS` environment
    /// variable as a comma-separated list.
    ///
    /// # Arguments
    ///
    /// * `origins` - A slice of string references representing allowed origins.
    ///
    /// # Returns
    ///
    /// The updated builder instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brom::BromApp;
    ///
    /// let app = BromApp::new().cors(&["http://localhost:8000", "https://example.com"]);
    /// ```
    #[must_use]
    pub fn cors(mut self, origins: &[&str]) -> Self {
        self.cors_origins = origins.iter().map(|s| (*s).to_string()).collect();
        self
    }

    /// Overrides the migrations directory path.
    ///
    /// By default, the application reads pending migrations from `"migrations"`.
    ///
    /// # Arguments
    ///
    /// * `path` - The folder path containing declarative migration SQL scripts.
    ///
    /// # Returns
    ///
    /// The updated builder instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brom::BromApp;
    ///
    /// let app = BromApp::new().migrations_dir("db/migrations");
    /// ```
    #[must_use]
    pub fn migrations_dir(mut self, path: &str) -> Self {
        self.migrations_dir = Some(path.to_string());
        self
    }

    /// Builds the application state and starts the HTTP server.
    ///
    /// Binds to `addr` and serves the dynamic REST endpoints, static embedded
    /// admin panel UI, and Swagger API documentation. Executes table migrations,
    /// compiles CORS policy, and registers entity schemas on startup.
    ///
    /// # Arguments
    ///
    /// * `addr` - The TCP socket address string to listen on (e.g. `"127.0.0.1:3000"`).
    ///
    /// # Returns
    ///
    /// A success result once the server shuts down, or a box error if binding or startup fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Database connection pool creation fails.
    /// * Running declarative table migrations fails.
    /// * Parsing the target TCP socket address fails.
    /// * Binding the listener to the specified address fails.
    ///
    /// # Panics
    ///
    /// Panics if the configured CORS origins list contains invalid HTTP header values.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use brom::BromApp;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     BromApp::new()
    ///         .serve("127.0.0.1:3000")
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn serve(self, addr: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        // 1. Resolve database path
        let db_path = self.db_path.unwrap_or_else(|| {
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "brom.db".to_string())
        });

        tracing::info!(db_path = %db_path, "initializing database");

        // 2. Create connection pool
        let pool = DbPool::new(&db_path)?;

        // 3. Run internal table migrations
        let runner = MigrationRunner::new(&pool);
        runner.ensure_internal_tables()?;

        // 4. Run user migrations
        let migrations_dir = self
            .migrations_dir
            .unwrap_or_else(|| "migrations".to_string());
        let applied = runner.run_pending(Path::new(&migrations_dir))?;
        if !applied.is_empty() {
            tracing::info!(count = applied.len(), "applied pending migrations");
        }

        // 5. Register schemas
        let registry = SchemaRegistry::new();
        for schema in &self.schemas {
            registry.register(schema.clone());
        }
        tracing::info!(count = self.schemas.len(), "registered entity schemas");

        // 6. Resolve CORS origins
        // #[allow(clippy::expect_used)] // Startup panics are acceptable per coding-standard §4.10
        let cors_origins: Vec<axum::http::HeaderValue> = if self.cors_origins.is_empty() {
            std::env::var("BROM_CORS_ORIGINS")
                .unwrap_or_default()
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| {
                    s.trim()
                        .parse::<axum::http::HeaderValue>()
                        .unwrap_or_else(|e| panic!("invalid CORS origin '{s}': {e}"))
                })
                .collect()
        } else {
            self.cors_origins
                .iter()
                .map(|s| {
                    s.parse::<axum::http::HeaderValue>()
                        .unwrap_or_else(|e| panic!("invalid CORS origin '{s}': {e}"))
                })
                .collect()
        };

        // 7. Load server config
        let secure_cookie = std::env::var("BROM_SECURE_COOKIE").map_or(true, |v| v != "false");
        let config = brom_server::ServerConfig {
            cors_origins: cors_origins.clone(),
            secure_cookie,
        };

        // 8. Construct application state
        let state = AppState {
            db: pool.clone(),
            session_store: Arc::new(pool.clone()),
            api_key_store: Arc::new(pool),
            schema_registry: Arc::new(registry),
            config,
        };

        // 9. Spawn background session cleanup
        brom_server::spawn_session_cleanup(state.session_store.clone());

        // 10. Build router and serve
        let router = brom_server::create_router(state, cors_origins);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!(addr = %addr, "brom server listening");
        axum::serve(listener, router).await?;

        Ok(())
    }
}

impl Default for BromApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use brom_core::{AuthPolicy, FieldInfo, FieldType, SchemaInfo};

    // Minimal EntitySchema impl for testing
    struct TestEntity;
    impl EntitySchema for TestEntity {
        fn table_name() -> &'static str {
            "test_entity"
        }
        fn fields() -> Vec<FieldInfo> {
            vec![FieldInfo {
                name: "id".to_string(),
                field_type: FieldType::Integer,
                constraints: vec![],
                ui_widget: None,
                hidden: false,
            }]
        }
        fn schema_info() -> SchemaInfo {
            SchemaInfo {
                table_name: "test_entity".to_string(),
                fields: Self::fields(),
                auth_policy: AuthPolicy::Public,
            }
        }
    }

    #[test]
    fn builder_registers_entities() {
        let app = BromApp::new().entity::<TestEntity>();
        assert_eq!(app.schemas.len(), 1);
        assert_eq!(app.schemas[0].table_name, "test_entity");
    }

    #[test]
    fn builder_defaults() {
        let app = BromApp::new();
        assert!(app.schemas.is_empty());
        assert!(app.db_path.is_none());
        assert!(app.migrations_dir.is_none());
    }

    #[test]
    fn builder_database_override() {
        let app = BromApp::new().database("custom.db");
        assert_eq!(app.db_path.as_deref(), Some("custom.db"));
    }

    #[test]
    fn builder_migrations_dir_override() {
        let app = BromApp::new().migrations_dir("my_migrations");
        assert_eq!(app.migrations_dir.as_deref(), Some("my_migrations"));
    }

    #[test]
    fn builder_cors_override() {
        let app = BromApp::new().cors(&["http://localhost:3000"]);
        assert_eq!(app.cors_origins, vec!["http://localhost:3000"]);
    }
}
