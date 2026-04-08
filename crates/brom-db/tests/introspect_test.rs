#![allow(clippy::expect_used)]
use brom_db::{DbPool, introspect::introspect_schema};

#[test]
fn test_introspect_empty_db() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Arrange: Initialize in-memory DB pool
    let pool = DbPool::in_memory()?;

    // 2. Act: Introspect the empty database
    let tables = introspect_schema(&pool)?;

    // 3. Assert: No user tables should be found (only _brom_ tables, which are filtered)
    assert!(
        tables.is_empty(),
        "Expected empty vector of tables, found: {tables:?}"
    );

    Ok(())
}

#[test]
fn test_introspect_single_table() -> Result<(), Box<dyn std::error::Error>> {
    let pool = DbPool::in_memory()?;
    let conn = pool.get()?;

    // 1. Arrange: Create a user table
    conn.execute(
        "CREATE TABLE posts (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            views INTEGER DEFAULT 0
        )",
        [],
    )?;

    // 2. Act
    let tables = introspect_schema(&pool)?;

    // 3. Assert
    assert_eq!(tables.len(), 1);
    let posts = &tables[0];
    assert_eq!(posts.name, "posts");
    assert_eq!(posts.columns.len(), 3);

    let id_col = posts
        .columns
        .iter()
        .find(|c| c.name == "id")
        .expect("id col");
    assert!(id_col.is_pk);
    assert_eq!(id_col.col_type, "TEXT");

    let title_col = posts
        .columns
        .iter()
        .find(|c| c.name == "title")
        .expect("title col");
    assert!(title_col.not_null);

    let views_col = posts
        .columns
        .iter()
        .find(|c| c.name == "views")
        .expect("views col");
    assert_eq!(views_col.default_value, Some("0".to_string()));

    Ok(())
}

#[test]
fn test_introspect_excludes_internal_tables() -> Result<(), Box<dyn std::error::Error>> {
    let pool = DbPool::in_memory()?;

    // 1. Arrange: Initialize internal tables
    let runner = brom_db::MigrationRunner::new(&pool);
    runner.ensure_internal_tables()?;

    // Create one user table
    let conn = pool.get()?;
    conn.execute("CREATE TABLE posts (id TEXT PRIMARY KEY)", [])?;

    // 2. Act
    let tables = introspect_schema(&pool)?;

    // 3. Assert: Only 'posts' should be present
    assert_eq!(tables.len(), 1);
    assert_eq!(tables[0].name, "posts");

    Ok(())
}

#[test]
fn test_introspect_foreign_keys() -> Result<(), Box<dyn std::error::Error>> {
    let pool = DbPool::in_memory()?;
    let conn = pool.get()?;

    // 1. Arrange: Create tables with FK
    conn.execute("CREATE TABLE authors (id TEXT PRIMARY KEY)", [])?;
    conn.execute(
        "CREATE TABLE posts (
            id TEXT PRIMARY KEY,
            author_id TEXT,
            FOREIGN KEY(author_id) REFERENCES authors(id)
        )",
        [],
    )?;

    // 2. Act
    let tables = introspect_schema(&pool)?;

    // 3. Assert
    let posts = tables
        .iter()
        .find(|t| t.name == "posts")
        .expect("posts table");
    assert_eq!(posts.foreign_keys.len(), 1);
    let fk = &posts.foreign_keys[0];
    assert_eq!(fk.from_column, "author_id");
    assert_eq!(fk.to_table, "authors");
    assert_eq!(fk.to_column, "id");

    Ok(())
}
