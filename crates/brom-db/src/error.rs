use thiserror::Error;

/// Database and persistence errors for the brom framework.
#[derive(Debug, Error)]
pub enum DbError {
    /// Failure initializing or maintaining database connection.
    #[error("connection error: {0}")]
    ConnectionError(String),

    /// Query syntax error or execution failure.
    #[error("query error: {0}")]
    QueryError(String),

    /// Errors encountered running schema migrations.
    #[error("migration error: {0}")]
    MigrationError(String),

    /// Pool exhaustions or configuration failures.
    #[error("pool error: {0}")]
    PoolError(String),

    /// General underlying IO failure.
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<rusqlite::Error> for DbError {
    fn from(err: rusqlite::Error) -> Self {
        Self::QueryError(err.to_string())
    }
}

impl From<r2d2::Error> for DbError {
    fn from(err: r2d2::Error) -> Self {
        Self::PoolError(err.to_string())
    }
}
