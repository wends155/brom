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
