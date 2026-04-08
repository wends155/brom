use crate::error::DbError;
use crate::pool::DbPool;
use serde::{Deserialize, Serialize};

/// A table as read from the live `SQLite` database via PRAGMAs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntrospectedTable {
    pub name: String,
    pub columns: Vec<IntrospectedColumn>,
    pub foreign_keys: Vec<IntrospectedForeignKey>,
}

/// A column as read from `PRAGMA table_info`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntrospectedColumn {
    pub name: String,
    pub col_type: String, // Normalized to uppercase ("TEXT", "INTEGER", etc)
    pub not_null: bool,
    pub default_value: Option<String>,
    pub is_pk: bool,
}

/// A foreign key as read from `PRAGMA foreign_key_list`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntrospectedForeignKey {
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
}

/// Reads all user-defined tables from the live database.
///
/// Excludes internal `_brom_*` tables and `sqlite_*` system tables.
///
/// # Errors
/// Returns `DbError::QueryError` if PRAGMA queries fail.
pub fn introspect_schema(pool: &DbPool) -> Result<Vec<IntrospectedTable>, DbError> {
    let conn = pool.get()?;

    // 1. Get all user tables names
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master 
         WHERE type='table' 
         AND name NOT LIKE 'sqlite_%' 
         AND name NOT LIKE '_brom_%'
         ORDER BY name ASC",
    )?;

    let table_names = stmt.query_map([], |row| row.get::<_, String>(0))?;

    let mut tables = Vec::new();

    for name_res in table_names {
        let name = name_res?;

        // 2. Get columns for this table
        // We use double quotes to escape table names that might be reserved words
        let mut col_stmt = conn.prepare(&format!("PRAGMA table_info(\"{name}\")"))?;
        let columns = col_stmt
            .query_map([], |row| {
                Ok(IntrospectedColumn {
                    name: row.get::<_, String>(1)?,
                    col_type: row.get::<_, String>(2)?.to_uppercase(),
                    not_null: row.get::<_, i32>(3)? != 0,
                    default_value: row.get::<_, Option<String>>(4)?,
                    is_pk: row.get::<_, i32>(5)? != 0,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;

        // 3. Get foreign keys for this table
        let mut fk_stmt = conn.prepare(&format!("PRAGMA foreign_key_list(\"{name}\")"))?;
        let foreign_keys = fk_stmt
            .query_map([], |row| {
                Ok(IntrospectedForeignKey {
                    from_column: row.get::<_, String>(3)?,
                    to_table: row.get::<_, String>(2)?,
                    to_column: row.get::<_, String>(4)?,
                })
            })?
            .collect::<Result<Vec<_>, rusqlite::Error>>()?;

        tables.push(IntrospectedTable {
            name,
            columns,
            foreign_keys,
        });
    }

    Ok(tables)
}
