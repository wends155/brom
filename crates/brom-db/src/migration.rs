use crate::error::DbError;
use crate::pool::DbPool;
use std::path::Path;

/// Runner for schema migrations.
pub struct MigrationRunner<'a> {
    pool: &'a DbPool,
}

impl<'a> MigrationRunner<'a> {
    /// Creates a new MigrationRunner.
    pub const fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Ensures all internal `_brom_*` tables exist.
    pub fn ensure_internal_tables(&self) -> Result<(), DbError> {
        let conn = self.pool.get()?;

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS _brom_user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS _brom_session (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL REFERENCES _brom_user(id) ON DELETE CASCADE,
                token TEXT NOT NULL UNIQUE,
                expires_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS _brom_api_key (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                user_id INTEGER NOT NULL REFERENCES _brom_user(id) ON DELETE CASCADE,
                key_hash TEXT NOT NULL UNIQUE,
                preview_hint TEXT NOT NULL,
                permissions TEXT NOT NULL,
                created_at TEXT NOT NULL,
                last_used_at TEXT
            );

            CREATE TABLE IF NOT EXISTS _brom_migration (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                version TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL
            );
            ",
        )?;

        Ok(())
    }

    /// Applies all pending migrations from the given directory.
    pub fn run_pending(&self, _migrations_dir: &Path) -> Result<Vec<String>, DbError> {
        // STUB(Phase 2): Implement directory reading and file application
        Ok(Vec::new())
    }
}
