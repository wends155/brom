use crate::error::DbError;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

/// A connection pool for `SQLite` databases.
#[derive(Clone)]
pub struct DbPool {
    pool: Pool<SqliteConnectionManager>,
}

impl DbPool {
    /// Creates a new database pool connected to the given path.
    ///
    /// # Errors
    /// Returns `DbError::PoolError` if connection manager fails to initialize.
    pub fn new(path: &str) -> Result<Self, DbError> {
        let manager = SqliteConnectionManager::file(path).with_init(|c| {
            c.execute_batch(
                "PRAGMA journal_mode = WAL;
                 PRAGMA synchronous = NORMAL;
                 PRAGMA foreign_keys = ON;",
            )
        });

        let pool = Pool::new(manager)?;
        Ok(Self { pool })
    }

    /// Creates an in-memory database pool, primarily for testing.
    ///
    /// # Errors
    /// Returns `DbError::PoolError` if connection manager fails to initialize.
    pub fn in_memory() -> Result<Self, DbError> {
        let manager = SqliteConnectionManager::memory().with_init(|c| {
            c.execute_batch(
                "PRAGMA journal_mode = WAL;
                 PRAGMA synchronous = NORMAL;
                 PRAGMA foreign_keys = ON;",
            )
        });

        let pool = Pool::new(manager)?;
        Ok(Self { pool })
    }

    /// Gets a pooled connection.
    ///
    /// # Errors
    /// Returns `DbError::PoolError` if a connection cannot be acquired before timeout.
    pub fn get(&self) -> Result<PooledConnection<SqliteConnectionManager>, DbError> {
        self.pool.get().map_err(Into::into)
    }
}
