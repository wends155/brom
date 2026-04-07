#![allow(clippy::expect_used)]
use brom_db::{DbPool, MigrationRunner};

#[test]
fn ensure_internal_tables_creates_all_four() {
    let pool = DbPool::in_memory().expect("pool");
    let runner = MigrationRunner::new(&pool);
    runner.ensure_internal_tables().expect("migration");

    let conn = pool.get().expect("conn");
    let tables: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '_brom_%'")
        .expect("prepare")
        .query_map([], |row| row.get(0))
        .expect("query")
        .filter_map(Result::ok)
        .collect();

    assert!(tables.contains(&"_brom_user".to_string()));
    assert!(tables.contains(&"_brom_session".to_string()));
    assert!(tables.contains(&"_brom_api_key".to_string()));
    assert!(tables.contains(&"_brom_migration".to_string()));
}

#[test]
fn ensure_internal_tables_is_idempotent() {
    let pool = DbPool::in_memory().expect("pool");
    let runner = MigrationRunner::new(&pool);
    runner.ensure_internal_tables().expect("first call");
    runner
        .ensure_internal_tables()
        .expect("second call should not fail");

    let conn = pool.get().expect("conn");
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name LIKE '_brom_%'",
            [],
            |row| row.get(0),
        )
        .expect("count");
    assert_eq!(count, 4);
}

#[test]
fn run_pending_rejects_invalid_filenames() {
    let pool = DbPool::in_memory().expect("pool");
    let runner = MigrationRunner::new(&pool);

    // Create a temporary directory for migrations
    let temp = std::env::temp_dir().join(format!("brom_test_{}", rand::random::<u32>()));
    std::fs::create_dir_all(&temp).expect("create dir");

    // Add an invalid filename
    std::fs::write(temp.join("invalid_format.sql"), "SELECT 1;").expect("write file");

    let result = runner.run_pending(&temp);
    assert!(result.is_err());
    let err = result
        .expect_err("should fail with path traversal error")
        .to_string();
    assert!(err.contains("invalid migration filename") || err.contains("structural validation"));

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp);
}
