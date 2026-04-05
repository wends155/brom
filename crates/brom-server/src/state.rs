use brom_auth::{ApiKeyStore, SessionStore};
use brom_core::schema::SchemaRegistry;
use brom_db::DbPool;
use std::sync::Arc;

/// Shared application state injected into all Axum handlers.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool.
    pub db: DbPool,
    /// Session store for admin authentication.
    pub session_store: Arc<dyn SessionStore>,
    /// API key store for programmatic access.
    pub api_key_store: Arc<dyn ApiKeyStore>,
    /// Static registry of entity schemas.
    pub schema_registry: Arc<SchemaRegistry>,
}
