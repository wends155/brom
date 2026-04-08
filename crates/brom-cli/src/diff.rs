use anyhow::Context;
use brom_core::{Constraint, FieldInfo, FieldType, SchemaInfo};
use brom_db::introspect::{IntrospectedTable};
use std::collections::{HashMap, HashSet, VecDeque};

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
    pub fn table_name(&self) -> &str {
        match self {
            MigrationOp::CreateTable { name, .. } => name,
            MigrationOp::AlterTableAddColumn { table_name, .. } => table_name,
            MigrationOp::DropColumn { table_name, .. } => table_name,
            MigrationOp::DropTable { name } => name,
        }
    }
}

pub struct DiffEngine {
    expected_schemas: Vec<SchemaInfo>,
    actual_tables: Vec<IntrospectedTable>,
}

impl DiffEngine {
    pub fn new(expected: Vec<SchemaInfo>, actual: Vec<IntrospectedTable>) -> Self {
        Self {
            expected_schemas: expected,
            actual_tables: actual,
        }
    }

    /// Compares expected entities against actual DB state and returns ordered migration operations.
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
        for (name, _) in &actual_map {
            if !expected_map.contains_key(name) {
                ops.push(MigrationOp::DropTable {
                    name: name.clone(),
                });
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

        Ok(self.topological_sort(ops))
    }

    fn topological_sort(&self, ops: Vec<MigrationOp>) -> Vec<MigrationOp> {
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

        let sorted_creates = self.sort_creates_by_dependency(creates);

        let mut final_ops = Vec::new();
        final_ops.extend(drops);
        final_ops.extend(sorted_creates);
        final_ops.extend(alters);
        final_ops.extend(col_drops);
        final_ops
    }

    fn sort_creates_by_dependency(&self, ops: Vec<MigrationOp>) -> Vec<MigrationOp> {
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

pub fn generate_migration_sql(ops: &[MigrationOp]) -> String {
    let mut up_sql = String::from("-- UP\n");
    let mut down_sql = String::from("\n-- DOWN\n");

    for op in ops {
        match op {
            MigrationOp::CreateTable { name, columns } => {
                up_sql.push_str(&format!("CREATE TABLE {} (\n", name));
                up_sql.push_str("    id TEXT PRIMARY KEY,\n");
                for col in columns {
                    if matches!(col.field_type, FieldType::ManyToMany { .. }) {
                        continue;
                    }
                    let col_sql = field_to_sql_definition(col);
                    up_sql.push_str(&format!("    {},\n", col_sql));
                }
                up_sql.push_str("    created_at TEXT DEFAULT (datetime('now')),\n");
                up_sql.push_str("    updated_at TEXT DEFAULT (datetime('now'))\n");
                up_sql.push_str(");\n");

                down_sql.push_str(&format!("DROP TABLE {};\n", name));
            }
            MigrationOp::AlterTableAddColumn { table_name, column } => {
                let col_sql = field_to_sql_definition(column);
                up_sql.push_str(&format!(
                    "ALTER TABLE {} ADD COLUMN {};\n",
                    table_name, col_sql
                ));
                down_sql.push_str(&format!(
                    "ALTER TABLE {} DROP COLUMN {};\n",
                    table_name, column.name
                ));
            }
            MigrationOp::DropColumn {
                table_name,
                column_name,
            } => {
                up_sql.push_str(&format!(
                    "ALTER TABLE {} DROP COLUMN {};\n",
                    table_name, column_name
                ));
                down_sql.push_str(&format!(
                    "-- TODO: Manual rollback needed for DropColumn {} on {}\n",
                    column_name, table_name
                ));
            }
            MigrationOp::DropTable { name } => {
                up_sql.push_str(&format!("DROP TABLE {};\n", name));
                down_sql.push_str(&format!("-- TODO: Manual rollback needed for DropTable {}\n", name));
            }
        }
    }

    format!("{}{}", up_sql, down_sql)
}

fn field_to_sql_definition(field: &FieldInfo) -> String {
    let type_sql = match &field.field_type {
        FieldType::String => "TEXT",
        FieldType::Integer => "INTEGER",
        FieldType::Float => "REAL",
        FieldType::Boolean => "INTEGER",
        FieldType::DateTime => "TEXT",
        FieldType::Link { .. } => "TEXT",
        FieldType::ManyToMany { .. } => "TEXT",
    };

    let mut def = format!("{} {}", field.name, type_sql);

    for constraint in &field.constraints {
        match constraint {
            Constraint::NotNull => def.push_str(" NOT NULL"),
            Constraint::Unique => def.push_str(" UNIQUE"),
            Constraint::Default(val) => def.push_str(&format!(" DEFAULT '{}'", val)),
        }
    }

    if let FieldType::Link { target } = &field.field_type {
        def.push_str(&format!(" REFERENCES {}(id)", target));
    }

    def
}
