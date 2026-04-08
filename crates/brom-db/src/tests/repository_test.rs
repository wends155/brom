#![allow(clippy::expect_used)]
use crate::DbPool;
use crate::SqliteRepository;
use brom_core::{
    AuthPolicy, EntitySchema, FieldInfo, FieldType, Pagination, Repository, SchemaInfo,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
}

impl EntitySchema for Post {
    fn table_name() -> &'static str {
        "test_posts"
    }
    fn fields() -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "id".into(),
                field_type: FieldType::Integer,
                constraints: vec![],
                ui_widget: None,
                hidden: false,
            },
            FieldInfo {
                name: "title".into(),
                field_type: FieldType::String,
                constraints: vec![],
                ui_widget: None,
                hidden: false,
            },
            FieldInfo {
                name: "body".into(),
                field_type: FieldType::String,
                constraints: vec![],
                ui_widget: None,
                hidden: false,
            },
        ]
    }
    fn schema_info() -> SchemaInfo {
        SchemaInfo {
            table_name: "test_posts".into(),
            fields: Self::fields(),
            auth_policy: AuthPolicy::Public,
        }
    }
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

#[test]
fn existing_schema_uses_valid_identifiers() {
    // Verify that all field names in the test Post schema are valid identifiers
    for field in Post::fields() {
        brom_core::validate_sql_identifier(&field.name).expect("field name should be valid");
    }
    brom_core::validate_sql_identifier(Post::table_name()).expect("table name should be valid");
}
