extern crate brom_core;
extern crate brom_macros;

use brom_db::{DbPool, SqliteRepository};
use brom_core::{EntitySchema, Pagination, Repository};
use serde::{Deserialize, Serialize};
use brom_macros::BromEntity;

#[derive(Debug, BromEntity, Serialize, Deserialize, Clone, PartialEq, utoipa::ToSchema)]
#[brom(table = "test_posts")]
struct Post {
    id: i64,
    title: String,
    content: String,
    created_at: Option<String>,
    updated_at: Option<String>,
}

fn setup_db() -> DbPool {
    let pool = DbPool::new(":memory:").expect("failed to create memory db");
    let conn = pool.get().expect("failed to get connection");
    conn.execute(
        "CREATE TABLE test_posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    ).expect("failed to create table");
    pool
}

#[test]
fn test_crud_lifecycle() {
    let pool = setup_db();
    let repo = SqliteRepository::<Post>::new(pool);

    // 1. Create
    let post = Post {
        id: 0,
        title: "Hello".to_string(),
        content: "World".to_string(),
        created_at: None,
        updated_at: None,
    };
    let id = repo.create(&post).expect("failed to create");
    assert_eq!(id, 1);

    // 2. Find by ID
    let found = repo.find_by_id(id).expect("failed to find").expect("not found");
    assert_eq!(found.title, "Hello");
    assert!(found.created_at.is_some());
    assert!(found.updated_at.is_some());
    let original_created_at = found.created_at.clone().unwrap();

    // 3. Update
    let mut to_update = found.clone();
    to_update.title = "Updated".to_string();
    repo.update(id, &to_update).expect("failed to update");

    let updated = repo.find_by_id(id).expect("failed to find").expect("not found");
    assert_eq!(updated.title, "Updated");
    assert_eq!(updated.created_at.as_ref().unwrap(), &original_created_at);
    assert!(updated.updated_at.is_some());

    // 4. Count
    let count = repo.count().expect("failed to count");
    assert_eq!(count, 1);

    // 5. Find All
    let all = repo.find_all(&Pagination { page: 1, per_page: 10 }).expect("failed to find all");
    assert_eq!(all.len(), 1);

    // 6. Delete
    repo.delete(id).expect("failed to delete");
    let count_after = repo.count().expect("failed to count");
    assert_eq!(count_after, 0);
}
