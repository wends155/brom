use thiserror::Error;

/// Core domain errors for the brom framework.
#[derive(Debug, Error)]
pub enum Error {
    #[error("schema error: {0}")]
    SchemaError(String),

    #[error("validation error for field '{field}': {message}")]
    ValidationError {
        field: String,
        message: String,
    },

    #[error("relation error: {0}")]
    RelationError(String),

    #[error("not found: entity {entity} with id {id}")]
    NotFound {
        entity: &'static str,
        id: i64,
    },
}
