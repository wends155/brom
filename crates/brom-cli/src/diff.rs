//! # Schema Diff Engine
//!
//! Provides the core logic for comparing declarative `BromEntity` schemas against
//! physical `SQLite` database schemas, synthesizing the necessary migration steps.

use brom_core::{Constraint, FieldInfo, FieldType, SchemaInfo};
use brom_db::introspect::IntrospectedTable;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Write;

/// Represents a discrete schema change operation required to transition
/// the database state closer to the expected entity declarations.
#[derive(Debug, Clone, PartialEq)]
pub enum MigrationOp {
    CreateTable {
        name: String,
        columns: Vec<FieldInfo>,
    },
    AlterTableAddColumn {
        table_name: String,
        column: FieldInfo,
    },
    DropColumn {
        table_name: String,
        column_name: String,
    },
    DropTable {
        name: String,
    },
}

impl MigrationOp {
    /// Extracts the primary table name targeted by this operation.
    ///
    /// # Returns
    ///
    /// A string slice representing the table name.
    pub fn table_name(&self) -> &str {
        match self {
            MigrationOp::CreateTable { name, .. } | MigrationOp::DropTable { name } => name,
            MigrationOp::AlterTableAddColumn { table_name, .. }
            | MigrationOp::DropColumn { table_name, .. } => table_name,
        }
    }
}

/// Engine responsible for schema synchronization and migration generation.
///
/// Holds the expected state (from code) and actual state (from DB) to compute a
/// minimal set of safe `MigrationOp` transitions.
pub struct DiffEngine {
    expected_schemas: Vec<SchemaInfo>,
    actual_tables: Vec<IntrospectedTable>,
}

impl DiffEngine {
    /// Constructs a new `DiffEngine`.
    ///
    /// # Arguments
    ///
    /// * `expected` - The intended schemas derived from code via `brom-macros`.
    /// * `actual` - The physical tables currently existing in the database.
    ///
    /// # Returns
    ///
    /// A configured `DiffEngine` ready for `diff()` execution.
    pub fn new(expected: Vec<SchemaInfo>, actual: Vec<IntrospectedTable>) -> Self {
        Self {
            expected_schemas: expected,
            actual_tables: actual,
        }
    }

    /// Compares expected entities against actual DB state and returns ordered migration operations.
    ///
    /// # Errors
    /// Returns an error if diffing logic fails for internal reasons.
    #[tracing::instrument(skip_all)]
    pub fn diff(&self) -> anyhow::Result<Vec<MigrationOp>> {
        let mut ops = Vec::new();

        let expected_map: HashMap<String, &SchemaInfo> = self
            .expected_schemas
            .iter()
            .map(|s| (s.table_name.clone(), s))
            .collect();

        let actual_map: HashMap<String, &IntrospectedTable> = self
            .actual_tables
            .iter()
            .map(|t| (t.name.clone(), t))
            .collect();

        // 1. New Tables
        for (name, expected) in &expected_map {
            if !actual_map.contains_key(name) {
                ops.push(MigrationOp::CreateTable {
                    name: name.clone(),
                    columns: expected.fields.clone(),
                });
            }
        }

        // 2. Deleted Tables
        for name in actual_map.keys() {
            if !expected_map.contains_key(name) {
                ops.push(MigrationOp::DropTable { name: name.clone() });
            }
        }

        // 3. Changed Tables (Columns only for now)
        for (name, expected) in &expected_map {
            if let Some(actual) = actual_map.get(name) {
                let actual_cols: HashSet<String> =
                    actual.columns.iter().map(|c| c.name.clone()).collect();
                let expected_cols: HashSet<String> =
                    expected.fields.iter().map(|f| f.name.clone()).collect();

                // New columns
                for field in &expected.fields {
                    // Skip ManyToMany as it's a junction table, not a column
                    if matches!(field.field_type, FieldType::ManyToMany { .. }) {
                        continue;
                    }
                    if !actual_cols.contains(&field.name) {
                        ops.push(MigrationOp::AlterTableAddColumn {
                            table_name: name.clone(),
                            column: field.clone(),
                        });
                    }
                }

                // Deleted columns (excluding auto-managed core fields)
                for col in &actual.columns {
                    if !expected_cols.contains(&col.name)
                        && col.name != "id"
                        && col.name != "created_at"
                        && col.name != "updated_at"
                    {
                        ops.push(MigrationOp::DropColumn {
                            table_name: name.clone(),
                            column_name: col.name.clone(),
                        });
                    }
                }
            }
        }

        Ok(Self::topological_sort(ops))
    }

    fn topological_sort(ops: Vec<MigrationOp>) -> Vec<MigrationOp> {
        let mut drops = Vec::new();
        let mut creates = Vec::new();
        let mut alters = Vec::new();
        let mut col_drops = Vec::new();

        for op in ops {
            match op {
                MigrationOp::DropTable { .. } => drops.push(op),
                MigrationOp::CreateTable { .. } => creates.push(op),
                MigrationOp::AlterTableAddColumn { .. } => alters.push(op),
                MigrationOp::DropColumn { .. } => col_drops.push(op),
            }
        }

        let sorted_creates = Self::sort_creates_by_dependency(creates);

        let mut final_ops = Vec::new();
        final_ops.extend(drops);
        final_ops.extend(sorted_creates);
        final_ops.extend(alters);
        final_ops.extend(col_drops);
        final_ops
    }

    fn sort_creates_by_dependency(ops: Vec<MigrationOp>) -> Vec<MigrationOp> {
        let mut adj = HashMap::new();
        let mut in_degree = HashMap::new();
        let mut op_map = HashMap::new();

        for op in &ops {
            if let MigrationOp::CreateTable { name, columns } = op {
                op_map.insert(name.clone(), op.clone());
                in_degree.entry(name.clone()).or_insert(0);

                for col in columns {
                    if let FieldType::Link { target } = &col.field_type {
                        // Only add dependency if target is also being created in this migration
                        if ops.iter().any(|o| o.table_name() == target) {
                            adj.entry(target.clone())
                                .or_insert_with(Vec::new)
                                .push(name.clone());
                            *in_degree.entry(name.clone()).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        let mut queue = VecDeque::new();
        for (name, _) in in_degree.iter().filter(|&(_, &d)| d == 0) {
            queue.push_back(name.clone());
        }

        let mut sorted = Vec::new();
        while let Some(u) = queue.pop_front() {
            if let Some(op) = op_map.get(&u) {
                sorted.push(op.clone());
            }
            if let Some(neighbors) = adj.get(&u) {
                for v in neighbors {
                    if let Some(degree) = in_degree.get_mut(v) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(v.clone());
                        }
                    }
                }
            }
        }

        // Cycle bridge: append anything missed (e.g. self-refs or circular refs)
        if sorted.len() < ops.len() {
            for op in ops {
                if !sorted.iter().any(|s| s.table_name() == op.table_name()) {
                    sorted.push(op);
                }
            }
        }

        sorted
    }
}

/// Transforms a sequence of migration operations into raw SQL statements.
///
/// # Arguments
///
/// * `ops` - The sequence of operations to translate.
///
/// # Returns
///
/// A tuple containing `(up_sql, down_sql)` corresponding to the forward
/// and rollback query batches.
#[tracing::instrument(skip_all)]
pub fn generate_migration_sql(ops: &[MigrationOp]) -> (String, String) {
    let mut up_sql = String::new();
    let mut down_sql = String::new();

    for op in ops {
        match op {
            MigrationOp::CreateTable { name, columns } => {
                let _ = writeln!(up_sql, "CREATE TABLE {name} (");
                let _ = writeln!(up_sql, "    id TEXT PRIMARY KEY,");
                for col in columns {
                    if matches!(col.field_type, FieldType::ManyToMany { .. }) {
                        continue;
                    }
                    let col_sql = field_to_sql_definition(col);
                    let _ = writeln!(up_sql, "    {col_sql},");
                }
                let _ = writeln!(up_sql, "    created_at TEXT DEFAULT (datetime('now')),");
                let _ = writeln!(up_sql, "    updated_at TEXT DEFAULT (datetime('now'))");
                let _ = writeln!(up_sql, ");");

                let _ = writeln!(down_sql, "DROP TABLE {name};");
            }
            MigrationOp::AlterTableAddColumn { table_name, column } => {
                let col_sql = field_to_sql_definition(column);
                let _ = writeln!(up_sql, "ALTER TABLE {table_name} ADD COLUMN {col_sql};");
                let _ = writeln!(
                    down_sql,
                    "ALTER TABLE {table_name} DROP COLUMN {};",
                    column.name
                );
            }
            MigrationOp::DropColumn {
                table_name,
                column_name,
            } => {
                let _ = writeln!(
                    up_sql,
                    "ALTER TABLE {table_name} DROP COLUMN {column_name};"
                );
                let _ = writeln!(
                    down_sql,
                    "-- TODO: Manual rollback needed for DropColumn {column_name} on {table_name}"
                );
            }
            MigrationOp::DropTable { name } => {
                let _ = writeln!(up_sql, "DROP TABLE {name};");
                let _ = writeln!(
                    down_sql,
                    "-- TODO: Manual rollback needed for DropTable {name}"
                );
            }
        }
    }

    (up_sql, down_sql)
}

fn field_to_sql_definition(field: &FieldInfo) -> String {
    let type_sql = match &field.field_type {
        FieldType::String
        | FieldType::DateTime
        | FieldType::Link { .. }
        | FieldType::ManyToMany { .. } => "TEXT",
        FieldType::Integer | FieldType::Boolean => "INTEGER",
        FieldType::Float => "REAL",
    };

    let mut def = format!("{} {}", field.name, type_sql);

    for constraint in &field.constraints {
        match constraint {
            Constraint::NotNull => def.push_str(" NOT NULL"),
            Constraint::Unique => def.push_str(" UNIQUE"),
            Constraint::Default(val) => {
                let _ = write!(def, " DEFAULT '{val}'");
            }
        }
    }

    if let FieldType::Link { target } = &field.field_type {
        let _ = write!(def, " REFERENCES {target}(id)");
    }

    def
}
#[cfg(test)]
mod tests {
    use super::*;
    use brom_core::entity::{FieldInfo, FieldType};
    use insta::assert_yaml_snapshot;

    #[test]
    fn test_generate_migration_sql_snapshots() {
        let ops = vec![
            MigrationOp::CreateTable {
                name: "posts".into(),
                columns: vec![
                    FieldInfo {
                        name: "title".into(),
                        field_type: FieldType::String,
                        constraints: vec![],
                        ui_widget: None,
                        hidden: false,
                    },
                    FieldInfo {
                        name: "author_id".into(),
                        field_type: FieldType::Link {
                            target: "users".into(),
                        },
                        constraints: vec![],
                        ui_widget: None,
                        hidden: false,
                    },
                ],
            },
            MigrationOp::AlterTableAddColumn {
                table_name: "users".into(),
                column: FieldInfo {
                    name: "bio".into(),
                    field_type: FieldType::String,
                    constraints: vec![],
                    ui_widget: None,
                    hidden: false,
                },
            },
            MigrationOp::DropColumn {
                table_name: "users".into(),
                column_name: "old_field".into(),
            },
            MigrationOp::DropTable {
                name: "obsolete_table".into(),
            },
        ];

        let (up, down) = generate_migration_sql(&ops);
        assert_yaml_snapshot!(up);
        assert_yaml_snapshot!(down);
    }
}
