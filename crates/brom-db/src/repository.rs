use crate::pool::DbPool;
use brom_core::{EntitySchema, Pagination, Repository};
use rusqlite::ErrorCode;
use rusqlite::types::Value as SqliteValue;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// `SQLite` implementation of the Repository trait.
pub struct SqliteRepository<T> {
    pool: DbPool,
    _marker: PhantomData<T>,
}

impl<T> SqliteRepository<T> {
    /// Creates a new generic repository with the given pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - An underlying database connection `DbPool`.
    ///
    /// # Returns
    ///
    /// A configured instance of `SqliteRepository` bound to the provided `DbPool`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brom_db::pool::DbPool;
    /// use brom_db::repository::SqliteRepository;
    /// use brom_core::EntitySchema;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User { id: i64 }
    /// // Impl EntitySchema for User...
    ///
    /// let pool = DbPool::in_memory().unwrap();
    /// let repo = SqliteRepository::<User>::new(pool);
    /// ```
    #[must_use]
    pub const fn new(pool: DbPool) -> Self {
        Self {
            pool,
            _marker: PhantomData,
        }
    }
}

impl<T: EntitySchema + Serialize + DeserializeOwned> Repository<T> for SqliteRepository<T> {
    #[tracing::instrument(skip_all)]
    fn create(&self, entity: &T) -> Result<i64, brom_core::Error> {
        let table = T::table_name();
        brom_core::validate_sql_identifier(table)?;
        let fields = T::fields();
        let columns: Vec<String> = fields
            .iter()
            .map(|f| f.name.clone())
            .filter(|name| name != "id")
            .collect();
        for col in &columns {
            brom_core::validate_sql_identifier(col)?;
        }
        let placeholders: Vec<String> = (1..=columns.len()).map(|_| "?".to_string()).collect();

        let sql = format!(
            "INSERT INTO {table} ({}) VALUES ({})",
            columns.join(", "),
            placeholders.join(", ")
        );

        let json =
            serde_json::to_value(entity).map_err(|e| brom_core::Error::Serde(e.to_string()))?;
        let obj = json
            .as_object()
            .ok_or_else(|| brom_core::Error::Serde("entity must be a JSON object".to_string()))?;

        let now = chrono::Utc::now().to_rfc3339();
        let params: Vec<SqliteValue> = columns
            .iter()
            .map(|col| {
                if col == "created_at" || col == "updated_at" {
                    SqliteValue::Text(now.clone())
                } else {
                    obj.get(col)
                        .cloned()
                        .map_or(SqliteValue::Null, json_to_sqlite_value)
                }
            })
            .collect();

        let conn = self
            .pool
            .get()
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;
        conn.execute(&sql, rusqlite::params_from_iter(params.clone()))
            .map_err(|e| match &e {
                rusqlite::Error::SqliteFailure(err, Some(msg))
                    if err.code == ErrorCode::ConstraintViolation
                        && msg.contains("UNIQUE constraint failed") =>
                {
                    if let Some(detail) = msg.split(": ").nth(1) {
                        let parts: Vec<&str> = detail.split('.').collect();
                        let col = if parts.len() == 2 { parts[1] } else { parts[0] };

                        let mut val_str = "unknown".to_string();
                        if let Some(val) = columns
                            .iter()
                            .position(|c| c == col)
                            .and_then(|idx| params.get(idx))
                        {
                            val_str = match val {
                                SqliteValue::Text(s) => s.clone(),
                                SqliteValue::Integer(i) => i.to_string(),
                                _ => format!("{val:?}"),
                            };
                        }

                        brom_core::Error::UniqueViolation {
                            entity: table.to_string(),
                            field: col.to_string(),
                            value: val_str,
                        }
                    } else {
                        brom_core::Error::Database(e.to_string())
                    }
                }
                _ => brom_core::Error::Database(e.to_string()),
            })?;

        Ok(conn.last_insert_rowid())
    }

    #[tracing::instrument(skip_all)]
    fn find_by_id(&self, id: i64) -> Result<Option<T>, brom_core::Error> {
        let table = T::table_name();
        brom_core::validate_sql_identifier(table)?;
        let sql = format!("SELECT * FROM {table} WHERE id = ?");

        let conn = self
            .pool
            .get()
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;

        let res = stmt.query_row([id], |row| {
            let mut map = serde_json::Map::new();
            let columns = row.as_ref().column_names();
            for (i, name) in columns.iter().enumerate() {
                let val: serde_json::Value = match row
                    .get_ref(i)
                    .map_err(|_| rusqlite::Error::InvalidColumnName((*name).to_string()))?
                {
                    rusqlite::types::ValueRef::Null => serde_json::Value::Null,
                    rusqlite::types::ValueRef::Integer(i) => serde_json::Value::Number(i.into()),
                    rusqlite::types::ValueRef::Real(f) => serde_json::Number::from_f64(f)
                        .map_or(serde_json::Value::Null, serde_json::Value::Number),
                    rusqlite::types::ValueRef::Text(t) => {
                        serde_json::Value::String(String::from_utf8_lossy(t).into_owned())
                    }
                    rusqlite::types::ValueRef::Blob(b) => serde_json::Value::String(hex::encode(b)),
                };
                map.insert((*name).to_string(), val);
            }
            Ok(serde_json::Value::Object(map))
        });

        match res {
            Ok(json) => {
                let entity = serde_json::from_value(json)
                    .map_err(|e| brom_core::Error::Serde(e.to_string()))?;
                Ok(Some(entity))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(brom_core::Error::Database(e.to_string())),
        }
    }

    #[tracing::instrument(skip_all)]
    fn find_all(&self, pagination: &Pagination) -> Result<Vec<T>, brom_core::Error> {
        let table = T::table_name();
        brom_core::validate_sql_identifier(table)?;
        let limit = pagination.per_page;
        let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
        let sql = format!("SELECT * FROM {table} LIMIT ? OFFSET ?");

        let conn = self
            .pool
            .get()
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;

        let rows = stmt
            .query_map([limit, offset], |row| {
                let mut map = serde_json::Map::new();
                for (i, name) in row.as_ref().column_names().iter().enumerate() {
                    let val: serde_json::Value = match row
                        .get_ref(i)
                        .map_err(|_| rusqlite::Error::InvalidColumnName((*name).to_string()))?
                    {
                        rusqlite::types::ValueRef::Null => serde_json::Value::Null,
                        rusqlite::types::ValueRef::Integer(i) => {
                            serde_json::Value::Number(i.into())
                        }
                        rusqlite::types::ValueRef::Real(f) => serde_json::Number::from_f64(f)
                            .map_or(serde_json::Value::Null, serde_json::Value::Number),
                        rusqlite::types::ValueRef::Text(t) => {
                            serde_json::Value::String(String::from_utf8_lossy(t).into_owned())
                        }
                        rusqlite::types::ValueRef::Blob(b) => {
                            serde_json::Value::String(hex::encode(b))
                        }
                    };
                    map.insert((*name).to_string(), val);
                }
                Ok(serde_json::Value::Object(map))
            })
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;

        let mut entities = Vec::new();
        for row in rows {
            let json = row.map_err(|e| brom_core::Error::Database(e.to_string()))?;
            let entity =
                serde_json::from_value(json).map_err(|e| brom_core::Error::Serde(e.to_string()))?;
            entities.push(entity);
        }

        Ok(entities)
    }

    #[tracing::instrument(skip_all)]
    fn update(&self, id: i64, entity: &T) -> Result<(), brom_core::Error> {
        let table = T::table_name();
        brom_core::validate_sql_identifier(table)?;
        let fields = T::fields();
        let set_clause = fields
            .iter()
            .map(|f| {
                brom_core::validate_sql_identifier(&f.name)?;
                Ok(format!("{} = ?", f.name))
            })
            .collect::<Result<Vec<_>, brom_core::Error>>()?
            .join(", ");
        let sql = format!("UPDATE {table} SET {set_clause} WHERE id = ?");

        let json =
            serde_json::to_value(entity).map_err(|e| brom_core::Error::Serde(e.to_string()))?;
        let obj = json
            .as_object()
            .ok_or_else(|| brom_core::Error::Serde("entity must be a JSON object".to_string()))?;

        let now = chrono::Utc::now().to_rfc3339();
        let mut params: Vec<SqliteValue> = fields
            .iter()
            .map(|f| {
                if f.name == "updated_at" {
                    SqliteValue::Text(now.clone())
                } else {
                    obj.get(&f.name)
                        .cloned()
                        .map_or(SqliteValue::Null, json_to_sqlite_value)
                }
            })
            .collect();
        params.push(SqliteValue::Integer(id));

        let conn = self
            .pool
            .get()
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;
        conn.execute(&sql, rusqlite::params_from_iter(params))
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    fn delete(&self, id: i64) -> Result<(), brom_core::Error> {
        let table = T::table_name();
        brom_core::validate_sql_identifier(table)?;
        let sql = format!("DELETE FROM {table} WHERE id = ?");

        let conn = self
            .pool
            .get()
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;
        conn.execute(&sql, [id])
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    fn count(&self) -> Result<i64, brom_core::Error> {
        let table = T::table_name();
        brom_core::validate_sql_identifier(table)?;
        let sql = format!("SELECT COUNT(*) FROM {table}");

        let conn = self
            .pool
            .get()
            .map_err(|e| brom_core::Error::Database(e.to_string()))?;
        conn.query_row(&sql, [], |row| row.get(0))
            .map_err(|e| brom_core::Error::Database(e.to_string()))
    }
}

fn json_to_sqlite_value(v: serde_json::Value) -> SqliteValue {
    match v {
        serde_json::Value::Null => SqliteValue::Null,
        serde_json::Value::Bool(b) => SqliteValue::Integer(i64::from(b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                SqliteValue::Integer(i)
            } else if let Some(f) = n.as_f64() {
                SqliteValue::Real(f)
            } else {
                SqliteValue::Null
            }
        }
        serde_json::Value::String(s) => SqliteValue::Text(s),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            SqliteValue::Text(v.to_string())
        }
    }
}
