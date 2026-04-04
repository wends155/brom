#![allow(clippy::expect_used)]
use crate::DbPool;
use crate::SqliteRepository;
use brom_core::{Pagination, Repository};
use brom_macros::BromEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, BromEntity, PartialEq)]
#[brom(table = "test_posts")]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
}

#[test]
fn test_sqlite_repository_crud() -> Result<(), Box<dyn std::error::Error>> {
    let pool = DbPool::in_memory()?;
    let runner = crate::MigrationRunner::new(&pool);
    runner.ensure_internal_tables()?;

    // Create table manually since we don't have automigrate for entities yet
    let conn = pool.get()?;
    conn.execute(
        "CREATE TABLE test_posts (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, body TEXT)",
        [],
    )?;

    let repo = SqliteRepository::<Post>::new(pool);
    let post = Post {
        id: 0,
        title: "Hello".into(),
        body: "World".into(),
    };

    // Test Create
    let id = repo.create(&post)?;
    assert!(id > 0);

    // Test Find by ID
    let found = repo.find_by_id(id)?.expect("Post should exist");
    assert_eq!(found.title, "Hello");
    assert_eq!(found.body, "World");

    // Test Find All
    let all = repo.find_all(&Pagination::default())?;
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].title, "Hello");

    // Test Update
    let mut updated = found.clone();
    updated.title = "Updated Title".into();
    repo.update(id, &updated)?;

    let found_updated = repo
        .find_by_id(id)?
        .expect("Post should exist after update");
    assert_eq!(found_updated.title, "Updated Title");

    // Test Count
    let count = repo.count()?;
    assert_eq!(count, 1);

    // Test Delete
    repo.delete(id)?;
    let deleted = repo.find_by_id(id)?;
    assert!(deleted.is_none());

    let final_count = repo.count()?;
    assert_eq!(final_count, 0);

    Ok(())
}
