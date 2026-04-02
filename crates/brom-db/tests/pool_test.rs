use brom_db::DbPool;

#[test]
fn in_memory_pool_connects() {
    let pool = DbPool::in_memory().expect("pool creation");
    let conn = pool.get().expect("connection");
    // Verify journal mode is memory for in-memory databases
    let mode: String = conn
        .query_row("PRAGMA journal_mode", [], |r| r.get(0))
        .expect("pragma");
    assert_eq!(mode, "memory");
}

