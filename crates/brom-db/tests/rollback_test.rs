use brom_db::{DbPool, MigrationRunner};
use std::fs;

#[test]
fn test_rollback_single_migration() -> Result<(), Box<dyn std::error::Error>> {
    let pool = DbPool::in_memory()?;
    let runner = MigrationRunner::new(&pool);
    runner.ensure_internal_tables()?;

    // 1. Arrange: Create a migration with UP and DOWN sections
    let temp = std::env::temp_dir().join(format!("brom_rollback_{}", rand::random::<u32>()));
    fs::create_dir_all(&temp)?;

    let migration_content = "-- UP\nCREATE TABLE logs (msg TEXT);\n-- DOWN\nDROP TABLE logs;";
    fs::write(temp.join("20240101_000000_init.sql"), migration_content)?;

    // 2. Act: Apply migration
    runner.run_pending(&temp)?;

    // Verify table exists
    {
        let conn = pool.get()?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='logs'",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(count, 1, "Table 'logs' should exist after migration");
    }

    // 3. Act: Rollback
    // Note: run_rollback doesn't exist yet
    runner.run_rollback(&temp)?;

    // 4. Assert: Table should be gone
    {
        let conn = pool.get()?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='logs'",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(count, 0, "Table 'logs' should be gone after rollback");
    }

    // Cleanup
    let _ = fs::remove_dir_all(&temp);
    Ok(())
}

#[test]
fn test_rollback_multi_step() -> Result<(), Box<dyn std::error::Error>> {
    let pool = DbPool::in_memory()?;
    let runner = MigrationRunner::new(&pool);
    runner.ensure_internal_tables()?;

    let temp = std::env::temp_dir().join(format!("brom_rollback_multi_{}", rand::random::<u32>()));
    fs::create_dir_all(&temp)?;

    // Migration 1
    fs::write(
        temp.join("20240101_000000_t1.sql"),
        "-- UP\nCREATE TABLE t1 (id TEXT);\n-- DOWN\nDROP TABLE t1;",
    )?;

    // Migration 2
    fs::write(
        temp.join("20240102_000000_t2.sql"),
        "-- UP\nCREATE TABLE t2 (id TEXT);\n-- DOWN\nDROP TABLE t2;",
    )?;

    runner.run_pending(&temp)?;

    // Rollback 1 (removes t2)
    runner.run_rollback(&temp)?;
    {
        let conn = pool.get()?;
        let t1_exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE name='t1'",
            [],
            |r| r.get(0),
        )?;
        let t2_exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE name='t2'",
            [],
            |r| r.get(0),
        )?;
        assert_eq!(t1_exists, 1);
        assert_eq!(t2_exists, 0);
    }

    // Rollback 2 (removes t1)
    runner.run_rollback(&temp)?;
    {
        let conn = pool.get()?;
        let t1_exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE name='t1'",
            [],
            |r| r.get(0),
        )?;
        assert_eq!(t1_exists, 0);
    }

    let _ = fs::remove_dir_all(&temp);
    Ok(())
}
