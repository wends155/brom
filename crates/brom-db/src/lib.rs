//! Database persistence layer for the `brom` headless CMS framework.

/// Storage implementations for API keys.
pub mod api_key_store;
/// Database-specific error types and wrappers.
pub mod error;
/// SQLite schema introspection utilities.
pub mod introspect;
/// Automated migration runner and history tracking.
pub mod migration;
/// `SQLite` connection pooling and initialization logic.
pub mod pool;
/// Active-record style repository for entity persistence.
pub mod repository;
/// Session storage implementations.
pub mod session_store;

pub use error::DbError;
pub use introspect::{
    IntrospectedColumn, IntrospectedForeignKey, IntrospectedTable, introspect_schema,
};
pub use migration::MigrationRunner;
pub use pool::DbPool;
pub use repository::SqliteRepository;

#[cfg(test)]
mod tests;
