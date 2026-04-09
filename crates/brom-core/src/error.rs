use thiserror::Error;

/// Core domain errors for the brom framework.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("schema error: {0}")]
    SchemaError(String),

    #[error("validation error for field '{field}': {message}")]
    ValidationError { field: String, message: String },

    #[error("relation error: {0}")]
    RelationError(String),

    #[error("not found: entity {entity} with id {id}")]
    NotFound { entity: &'static str, id: i64 },

    #[error("database error: {0}")]
    Database(String),

    #[error("unique constraint violation: {entity} with {field}='{value}' already exists")]
    UniqueViolation {
        entity: String,
        field: String,
        value: String,
    },

    #[error("serialization error: {0}")]
    Serde(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_violation_display() {
        let err = Error::UniqueViolation {
            entity: "Post".to_string(),
            field: "slug".to_string(),
            value: "hello-world".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "unique constraint violation: Post with slug='hello-world' already exists"
        );
    }
}
