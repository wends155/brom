use brom_core::{EntitySchema, Pagination, Repository};
use std::marker::PhantomData;

/// `SQLite` implementation of the Repository trait.
pub struct SqliteRepository<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for SqliteRepository<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SqliteRepository<T> {
    /// Creates a new generic repository.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: EntitySchema> Repository<T> for SqliteRepository<T> {
    #[tracing::instrument(skip_all)]
    fn create(&self, _entity: &T) -> Result<i64, brom_core::Error> {
        // STUB(Phase 2): Replace with real CRUD implementation using rusqlite
        Err(brom_core::Error::SchemaError("not yet implemented".into()))
    }

    #[tracing::instrument(skip_all)]
    fn find_by_id(&self, _id: i64) -> Result<Option<T>, brom_core::Error> {
        Err(brom_core::Error::SchemaError("not yet implemented".into()))
    }

    #[tracing::instrument(skip_all)]
    fn find_all(&self, _pagination: &Pagination) -> Result<Vec<T>, brom_core::Error> {
        Err(brom_core::Error::SchemaError("not yet implemented".into()))
    }

    #[tracing::instrument(skip_all)]
    fn update(&self, _id: i64, _entity: &T) -> Result<(), brom_core::Error> {
        Err(brom_core::Error::SchemaError("not yet implemented".into()))
    }

    #[tracing::instrument(skip_all)]
    fn delete(&self, _id: i64) -> Result<(), brom_core::Error> {
        Err(brom_core::Error::SchemaError("not yet implemented".into()))
    }

    #[tracing::instrument(skip_all)]
    fn count(&self) -> Result<i64, brom_core::Error> {
        Err(brom_core::Error::SchemaError("not yet implemented".into()))
    }
}
