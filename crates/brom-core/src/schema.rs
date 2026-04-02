use crate::entity::SchemaInfo;
use std::sync::RwLock;

/// Global registry of all schemas defined in the application.
pub struct SchemaRegistry {
    schemas: RwLock<Vec<SchemaInfo>>,
}

impl SchemaRegistry {
    /// Creates a new, empty registry.
    pub const fn new() -> Self {
        Self {
            schemas: RwLock::new(Vec::new()),
        }
    }

    /// Registers a new schema.
    pub fn register(&self, schema: SchemaInfo) {
        if let Ok(mut lock) = self.schemas.write() {
            lock.push(schema);
        }
    }

    /// Returns a copy of all registered schemas.
    pub fn all_schemas(&self) -> Vec<SchemaInfo> {
        if let Ok(lock) = self.schemas.read() {
            lock.clone()
        } else {
            Vec::new()
        }
    }
}
