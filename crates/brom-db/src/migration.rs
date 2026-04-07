use crate::error::DbError;
use crate::pool::DbPool;
use rusqlite::OptionalExtension;
use std::path::Path;

/// Runner for schema migrations.
pub struct MigrationRunner<'a> {
    pool: &'a DbPool,
}

impl<'a> MigrationRunner<'a> {
    /// Creates a new `MigrationRunner`.
    #[must_use]
    pub const fn new(pool: &'a DbPool) -> Self {
        Self { pool }
    }

    /// Ensures all internal `_brom_*` tables exist.
    ///
    /// # Errors
    /// Returns `DbError` if table creation fails.
    #[tracing::instrument(skip_all)]
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
                key_prefix TEXT NOT NULL,
                permissions TEXT NOT NULL,
                created_at TEXT NOT NULL,
                last_used_at TEXT
            );

            CREATE TABLE IF NOT EXISTS _brom_migration (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                version TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL,
                checksum TEXT NOT NULL
            );
            ",
        )?;

        Ok(())
    }

    /// Applies all pending migrations from the given directory.
    ///
    /// # Errors
    /// Returns `DbError` if reading migrations or executing them fails.
    #[tracing::instrument(skip_all)]
    pub fn run_pending(&self, migrations_dir: &Path) -> Result<Vec<String>, DbError> {
        use sha2::{Digest, Sha256};

        if !migrations_dir.exists() {
            return Ok(Vec::new());
        }

        let mut entries: Vec<_> = std::fs::read_dir(migrations_dir)?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "sql"))
            .collect();

        entries.sort_by_key(std::fs::DirEntry::file_name);

        let mut conn = self.pool.get()?;
        let tx = conn
            .transaction()
            .map_err(|e| DbError::PoolError(e.to_string()))?;

        let mut applied = Vec::new();
        for entry in entries {
            let path = entry.path();
            let version = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| DbError::PoolError("invalid migration filename".into()))?
                .to_string();

            let sql = std::fs::read_to_string(&path).map_err(|e| {
                DbError::PoolError(format!("failed to read migration {version}: {e}"))
            })?;
            let checksum = format!("{:x}", Sha256::digest(sql.as_bytes()));

            let row: Option<String> = tx
                .query_row(
                    "SELECT checksum FROM _brom_migration WHERE version = ?",
                    [&version],
                    |row| row.get(0),
                )
                .optional()
                .map_err(|e: rusqlite::Error| DbError::PoolError(e.to_string()))?;

            if let Some(stored_checksum) = row {
                if stored_checksum != checksum {
                    return Err(DbError::MigrationError(format!(
                        "checksum mismatch for migration '{version}': expected {stored_checksum}, got {checksum}"
                    )));
                }
            } else {
                // Extract human-readable name from filename
                // e.g., "20260406_120000_add_posts" -> "add_posts"
                let name = version
                    .splitn(3, '_')
                    .nth(2)
                    .unwrap_or(&version)
                    .to_string();

                tx.execute_batch(&sql).map_err(|e| {
                    DbError::PoolError(format!("failed to execute migration {version}: {e}"))
                })?;
                tx.execute(
                    "INSERT INTO _brom_migration (version, name, applied_at, checksum) VALUES (?, ?, datetime('now'), ?)",
                    [&version, &name, &checksum],
                )
                .map_err(|e| DbError::PoolError(e.to_string()))?;
                applied.push(version);
            }
        }

        tx.commit().map_err(|e| DbError::PoolError(e.to_string()))?;
        Ok(applied)
    }
}
