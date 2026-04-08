#![allow(clippy::unwrap_used, clippy::expect_used)]
use brom_cli::diff::{DiffEngine, MigrationOp};
use brom_core::{AuthPolicy, Constraint, FieldInfo, FieldType, SchemaInfo};
use brom_db::{DbPool, MigrationRunner, introspect_schema};
use std::fs;

#[test]
fn test_diff_new_table() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Arrange: Define the "expected" schema (one table)
    let expected = vec![SchemaInfo {
        table_name: "posts".to_string(),
        fields: vec![FieldInfo {
            name: "title".to_string(),
            field_type: FieldType::String,
            constraints: vec![Constraint::NotNull],
            ui_widget: None,
            hidden: false,
        }],
        auth_policy: AuthPolicy::Public,
    }];

    // actual db is empty
    let pool = DbPool::in_memory()?;
    let actual = introspect_schema(&pool)?;

    // 2. Act: Run diff
    let engine = DiffEngine::new(expected, actual);
    let ops = engine.diff()?;

    // 3. Assert: Should have a "CreateTable" operation for 'posts'
    assert_eq!(ops.len(), 1);
    match &ops[0] {
        MigrationOp::CreateTable { name, .. } => assert_eq!(name, "posts"),
        other => panic!("Expected CreateTable, found {other:?}"),
    }

    Ok(())
}

#[test]
fn test_diff_add_column() -> Result<(), Box<dyn std::error::Error>> {
    let pool = DbPool::in_memory()?;
    let conn = pool.get()?;
    conn.execute("CREATE TABLE posts (id TEXT PRIMARY KEY)", [])?;

    let expected = vec![SchemaInfo {
        table_name: "posts".to_string(),
        fields: vec![FieldInfo {
            name: "title".to_string(),
            field_type: FieldType::String,
            constraints: vec![Constraint::NotNull],
            ui_widget: None,
            hidden: false,
        }],
        auth_policy: AuthPolicy::Public,
    }];

    let actual = introspect_schema(&pool)?;
    let engine = DiffEngine::new(expected, actual);
    let ops = engine.diff()?;

    assert_eq!(ops.len(), 1);
    match &ops[0] {
        MigrationOp::AlterTableAddColumn { table_name, column } => {
            assert_eq!(table_name, "posts");
            assert_eq!(column.name, "title");
        }
        _ => panic!("Expected AlterTableAddColumn"),
    }

    Ok(())
}

#[test]
fn test_diff_drop_column() -> Result<(), Box<dyn std::error::Error>> {
    let pool = DbPool::in_memory()?;
    let conn = pool.get()?;
    conn.execute(
        "CREATE TABLE posts (id TEXT PRIMARY KEY, obsolete TEXT)",
        [],
    )?;

    let expected = vec![SchemaInfo {
        table_name: "posts".to_string(),
        fields: vec![],
        auth_policy: AuthPolicy::Public,
    }];

    let actual = introspect_schema(&pool)?;
    let engine = DiffEngine::new(expected, actual);
    let ops = engine.diff()?;

    // Should drop 'obsolete', but NOT 'id'
    assert_eq!(ops.len(), 1);
    match &ops[0] {
        MigrationOp::DropColumn {
            table_name,
            column_name,
        } => {
            assert_eq!(table_name, "posts");
            assert_eq!(column_name, "obsolete");
        }
        _ => panic!("Expected DropColumn"),
    }

    Ok(())
}

#[test]
fn test_diff_topological_sort() -> Result<(), Box<dyn std::error::Error>> {
    // authors must be created before posts because posts links to authors
    let expected = vec![
        SchemaInfo {
            table_name: "posts".to_string(),
            fields: vec![FieldInfo {
                name: "author_id".to_string(),
                field_type: FieldType::Link {
                    target: "authors".to_string(),
                },
                constraints: vec![],
                ui_widget: None,
                hidden: false,
            }],
            auth_policy: AuthPolicy::Public,
        },
        SchemaInfo {
            table_name: "authors".to_string(),
            fields: vec![],
            auth_policy: AuthPolicy::Public,
        },
    ];

    let actual = vec![]; // empty db
    let engine = DiffEngine::new(expected, actual);
    let ops = engine.diff()?;

    assert_eq!(ops.len(), 2);
    // authors should be first
    match (&ops[0], &ops[1]) {
        (MigrationOp::CreateTable { name: n1, .. }, MigrationOp::CreateTable { name: n2, .. }) => {
            assert_eq!(n1, "authors");
            assert_eq!(n2, "posts");
        }
        _ => panic!("Expected two CreateTable ops ordered by dependency"),
    }

    Ok(())
}

#[test]
fn test_round_trip_migration_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let pool = DbPool::in_memory()?;
    let runner = MigrationRunner::new(&pool);
    runner.ensure_internal_tables()?;

    // Setup temp migrations dir
    let temp = std::env::temp_dir().join(format!("brom_roundtrip_{}", rand::random::<u32>()));
    fs::create_dir_all(&temp)?;

    // --- STEP 1: Initial Schema ---
    let schema_v1 = vec![SchemaInfo {
        table_name: "users".to_string(),
        fields: vec![FieldInfo {
            name: "username".to_string(),
            field_type: FieldType::String,
            constraints: vec![Constraint::NotNull],
            ui_widget: None,
            hidden: false,
        }],
        auth_policy: AuthPolicy::Public,
    }];

    // Diff against empty DB
    let live_v0 = introspect_schema(&pool)?;
    let engine_v1 = DiffEngine::new(schema_v1.clone(), live_v0);
    let ops_v1 = engine_v1.diff()?;
    assert_eq!(ops_v1.len(), 1); // CreateTable users

    let (up_v1, down_v1) = brom_cli::diff::generate_migration_sql(&ops_v1);
    fs::write(
        temp.join("20240101_000000_init.sql"),
        format!("-- UP\n{up_v1}\n-- DOWN\n{down_v1}"),
    )?;

    // Apply
    runner.run_pending(&temp)?;

    // Verify State
    let live_v1 = introspect_schema(&pool)?;
    assert_eq!(live_v1.len(), 1);
    assert_eq!(live_v1[0].name, "users");

    // --- STEP 2: Modified Schema (Add Column) ---
    let schema_v2 = vec![SchemaInfo {
        table_name: "users".to_string(),
        fields: vec![
            FieldInfo {
                name: "username".to_string(),
                field_type: FieldType::String,
                constraints: vec![Constraint::NotNull],
                ui_widget: None,
                hidden: false,
            },
            FieldInfo {
                name: "bio".to_string(),
                field_type: FieldType::String,
                constraints: vec![],
                ui_widget: None,
                hidden: false,
            },
        ],
        auth_policy: AuthPolicy::Public,
    }];

    let engine_v2 = DiffEngine::new(schema_v2, live_v1);
    let ops_v2 = engine_v2.diff()?;
    assert_eq!(ops_v2.len(), 1); // AlterTableAddColumn bio

    let (up_v2, down_v2) = brom_cli::diff::generate_migration_sql(&ops_v2);
    fs::write(
        temp.join("20240102_000000_bio.sql"),
        format!("-- UP\n{up_v2}\n-- DOWN\n{down_v2}"),
    )?;

    // Apply
    runner.run_pending(&temp)?;

    // Verify State
    let live_v2 = introspect_schema(&pool)?;
    let user_table = live_v2.iter().find(|t| t.name == "users").unwrap();
    assert!(user_table.columns.iter().any(|c| c.name == "bio"));

    // --- STEP 3: Rollback ---
    runner.run_rollback(&temp)?; // Rollback bio
    let live_v3 = introspect_schema(&pool)?;
    let user_table_v3 = live_v3.iter().find(|t| t.name == "users").unwrap();
    assert!(
        !user_table_v3.columns.iter().any(|c| c.name == "bio"),
        "bio column should be gone"
    );

    runner.run_rollback(&temp)?; // Rollback users table
    let live_v4 = introspect_schema(&pool)?;
    assert_eq!(
        live_v4.len(),
        0,
        "DB should be empty after rolling back both migrations"
    );

    let _ = fs::remove_dir_all(&temp);
    Ok(())
}
