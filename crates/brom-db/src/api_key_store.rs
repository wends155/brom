use crate::pool::DbPool;
use brom_auth::{ApiKeyRecord, ApiKeyStore, AuthError};
use chrono::Utc;
use rand::RngCore;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};

impl ApiKeyStore for DbPool {
    fn create(
        &self,
        user_id: i64,
        name: &str,
        permissions: &str,
    ) -> Result<(String, ApiKeyRecord), AuthError> {
        let conn = self
            .get()
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        // Generate a 32-byte secure random key
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes);
        let raw_key = hex::encode(key_bytes);

        // Hash for storage (SHA-256)
        let mut hasher = Sha256::new();
        hasher.update(raw_key.as_bytes());
        let key_hash = hex::encode(hasher.finalize());

        // Use first 8 chars as prefix/hint
        let key_prefix = raw_key.chars().take(8).collect::<String>();
        let created_at = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO _brom_api_key (name, user_id, key_hash, key_prefix, permissions, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (name, user_id, &key_hash, &key_prefix, permissions, &created_at),
        ).map_err(|e| AuthError::InternalError(e.to_string()))?;

        let id = conn.last_insert_rowid();

        Ok((
            raw_key,
            ApiKeyRecord {
                id,
                name: name.to_string(),
                key_prefix,
                permissions: permissions.to_string(),
                user_id,
                created_at,
                last_used_at: None,
            },
        ))
    }

    fn validate(&self, raw_key: &str) -> Result<ApiKeyRecord, AuthError> {
        let conn = self
            .get()
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        // Compute hash of provided key
        let mut hasher = Sha256::new();
        hasher.update(raw_key.as_bytes());
        let computed_hash = hex::encode(hasher.finalize());

        let mut record = conn
            .query_row(
                "SELECT id, name, key_prefix, permissions, user_id, created_at, last_used_at 
             FROM _brom_api_key WHERE key_hash = ?1",
                [computed_hash],
                |row| {
                    Ok(ApiKeyRecord {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        key_prefix: row.get(2)?,
                        permissions: row.get(3)?,
                        user_id: row.get(4)?,
                        created_at: row.get(5)?,
                        last_used_at: row.get(6)?,
                    })
                },
            )
            .map_err(|_| AuthError::InvalidApiKey)?;

        // Update last_used_at
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE _brom_api_key SET last_used_at = ?1 WHERE id = ?2",
            (&now, record.id),
        )
        .ok(); // Non-critical if this fails, don't block

        record.last_used_at = Some(now);

        Ok(record)
    }

    fn revoke(&self, id: i64, user_id: i64) -> Result<(), AuthError> {
        let conn = self
            .get()
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        let rows = conn
            .execute(
                "DELETE FROM _brom_api_key WHERE id = ?1 AND user_id = ?2",
                (id, user_id),
            )
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        if rows == 0 {
            return Err(AuthError::InvalidApiKey);
        }

        Ok(())
    }

    fn list_for_user(&self, user_id: i64) -> Result<Vec<ApiKeyRecord>, AuthError> {
        let conn = self
            .get()
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        let mut stmt = conn
            .prepare(
                "SELECT id, name, key_prefix, permissions, user_id, created_at, last_used_at 
             FROM _brom_api_key WHERE user_id = ?1 ORDER BY created_at DESC",
            )
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        let records = stmt
            .query_map([user_id], |row| {
                Ok(ApiKeyRecord {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    key_prefix: row.get(2)?,
                    permissions: row.get(3)?,
                    user_id: row.get(4)?,
                    created_at: row.get(5)?,
                    last_used_at: row.get(6)?,
                })
            })
            .map_err(|e| AuthError::InternalError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        Ok(records)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::migration::MigrationRunner;
    use crate::pool::DbPool;

    fn setup_test_db() -> DbPool {
        let db_name = format!(
            "file:memdb_apikey_{}?mode=memory&cache=shared",
            rand::random::<u32>()
        );
        let pool = DbPool::new(&db_name).expect("Failed to create in-memory DB");
        let runner = MigrationRunner::new(&pool);
        runner
            .ensure_internal_tables()
            .expect("Failed to run internal migrations");

        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO _brom_user (email, password_hash, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            (format!("test_{}@example.com", rand::random::<u32>()), "hash", "2026-01-01T00:00:00Z", "2026-01-01T00:00:00Z"),
        ).unwrap();

        pool
    }

    #[test]
    fn test_api_key_lifecycle() {
        let pool = setup_test_db();

        // Create key
        let (raw, record) = pool
            .create(1, "test-key", "read_write")
            .expect("Failed to create key");
        assert_eq!(record.name, "test-key");
        assert_eq!(record.permissions, "read_write");
        assert_eq!(record.key_prefix.len(), 8);

        // Validate key
        let validated = pool.validate(&raw).expect("Failed to validate key");
        assert_eq!(validated.id, record.id);
        assert!(validated.last_used_at.is_some());

        // List for user
        let keys = pool.list_for_user(1).expect("Failed to list keys");
        assert_eq!(keys.len(), 1);

        // Revoke key
        pool.revoke(record.id, 1).expect("Failed to revoke key");

        // Validate should fail
        let result = pool.validate(&raw);
        assert!(matches!(result, Err(AuthError::InvalidApiKey)));

        // List should be empty
        let keys = pool.list_for_user(1).expect("Failed to list keys");
        assert_eq!(keys.len(), 0);
    }
}
