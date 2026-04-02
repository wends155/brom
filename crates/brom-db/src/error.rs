use thiserror::Error;

/// Database and persistence errors for the brom framework.
#[derive(Debug, Error)]
pub enum DbError {
    #[error("connection error: {0}")]
    ConnectionError(String),

    #[error("query error: {0}")]
    QueryError(String),

    #[error("migration error: {0}")]
    MigrationError(String),

    #[error("pool error: {0}")]
    PoolError(String),
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
