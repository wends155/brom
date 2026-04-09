use crate::entity::SchemaInfo;
use std::sync::RwLock;

/// Global registry of all schemas defined in the application.
pub struct SchemaRegistry {
    schemas: RwLock<Vec<SchemaInfo>>,
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaRegistry {
    /// Creates a new, empty registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            schemas: RwLock::new(Vec::new()),
        }
    }

    /// Registers a new schema.
    pub fn register(&self, schema: SchemaInfo) {
        let mut lock = self
            .schemas
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        if lock.iter().any(|s| s.table_name == schema.table_name) {
            tracing::warn!(
                table_name = %schema.table_name,
                "schema already registered — skipping duplicate"
            );
            return;
        }

        lock.push(schema);
    }

    /// Returns a copy of all registered schemas.
    pub fn all_schemas(&self) -> Vec<SchemaInfo> {
        let lock = self
            .schemas
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        lock.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuthPolicy, FieldInfo, FieldType};

    fn sample_schema(name: &str) -> SchemaInfo {
        SchemaInfo {
            table_name: name.to_string(),
            fields: vec![FieldInfo {
                name: "id".to_string(),
                field_type: FieldType::Integer,
                constraints: vec![],
                ui_widget: None,
                hidden: false,
            }],
            auth_policy: AuthPolicy::Public,
        }
    }

    #[test]
    fn register_and_retrieve() {
        let reg = SchemaRegistry::new();
        reg.register(sample_schema("post"));
        reg.register(sample_schema("tag"));
        let schemas = reg.all_schemas();
        assert_eq!(schemas.len(), 2);
    }

    #[test]
    fn register_duplicate_is_idempotent() {
        let reg = SchemaRegistry::new();
        reg.register(sample_schema("post"));
        reg.register(sample_schema("post"));
        assert_eq!(reg.all_schemas().len(), 1);
    }

    #[test]
    fn empty_registry_returns_empty_vec() {
        let reg = SchemaRegistry::new();
        assert!(reg.all_schemas().is_empty());
    }
}
