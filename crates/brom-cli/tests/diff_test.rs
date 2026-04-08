use brom_db::{DbPool, introspect_schema};
use brom_cli::diff::{DiffEngine, MigrationOp};
use brom_core::{SchemaInfo, FieldInfo, FieldType, Constraint, AuthPolicy};

#[test]
fn test_diff_new_table() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Arrange: Define the "expected" schema (one table)
    let expected = vec![SchemaInfo {
        table_name: "posts".to_string(),
        fields: vec![
            FieldInfo {
                name: "title".to_string(),
                field_type: FieldType::String,
                constraints: vec![Constraint::NotNull],
                ui_widget: None,
                hidden: false,
            },
        ],
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
        other => panic!("Expected CreateTable, found {:?}", other),
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
        fields: vec![
            FieldInfo {
                name: "title".to_string(),
                field_type: FieldType::String,
                constraints: vec![Constraint::NotNull],
                ui_widget: None,
                hidden: false,
            },
        ],
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
    conn.execute("CREATE TABLE posts (id TEXT PRIMARY KEY, obsolete TEXT)", [])?;
    
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
        MigrationOp::DropColumn { table_name, column_name } => {
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
            fields: vec![
                FieldInfo {
                    name: "author_id".to_string(),
                    field_type: FieldType::Link { target: "authors".to_string() },
                    constraints: vec![],
                    ui_widget: None,
                    hidden: false,
                },
            ],
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
