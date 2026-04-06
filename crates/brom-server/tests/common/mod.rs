#![allow(clippy::unwrap_used, clippy::expect_used)]

use brom_core::schema::SchemaRegistry;
use brom_db::{DbPool, MigrationRunner};
use brom_server::AppState;
use std::sync::Arc;

/// Builds a fully-wired `AppState` backed by in-memory `SQLite`.
///
/// Runs internal table migrations so all _brom_ tables exist.
pub fn test_app_state() -> AppState {
    let pool = DbPool::in_memory().expect("in-memory pool");
    let runner = MigrationRunner::new(&pool);
    runner.ensure_internal_tables().expect("migrations");

    AppState {
        db: pool.clone(),
        session_store: Arc::new(pool.clone()),
        api_key_store: Arc::new(pool.clone()),
        schema_registry: Arc::new(SchemaRegistry::new()),
    }
}

/// Seeds an admin user with known credentials.
/// Returns (`user_id`, `raw_password`).
pub fn seed_admin_user(state: &AppState) -> (i64, String) {
    let password = "test_password_123";
    let hash = brom_auth::password::hash_password(password).expect("hash");
    let conn = state.db.get().expect("conn");
    conn.execute(
        "INSERT INTO _brom_user (email, password_hash, created_at, updated_at) \
         VALUES (?1, ?2, '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z')",
        ("admin@test.com", &hash),
    )
    .expect("seed user");
    let user_id = conn.last_insert_rowid();
    (user_id, password.to_string())
}

/// Creates a real session via the `SessionStore` trait and returns the token.
pub fn create_test_session(state: &AppState, user_id: i64) -> String {
    state
        .session_store
        .create(user_id)
        .expect("create session")
        .token
}
