use crate::pool::DbPool;
use brom_auth::{AuthError, Session, SessionStore};
use chrono::{DateTime, Duration, Utc};
use rand::RngCore;
use rand::rngs::OsRng;

impl SessionStore for DbPool {
    fn create(&self, user_id: i64) -> Result<Session, AuthError> {
        let conn = self
            .get()
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        // Generate a 32-byte secure random token (hex encoded)
        let mut token_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut token_bytes);
        let token = hex::encode(token_bytes);

        // Default expiry: 24 hours
        let expires_at = Utc::now() + Duration::hours(24);
        let expires_at_str = expires_at.to_rfc3339();

        conn.execute(
            "INSERT INTO _brom_session (user_id, token, expires_at) VALUES (?1, ?2, ?3)",
            (user_id, &token, &expires_at_str),
        )
        .map_err(|e| AuthError::InternalError(e.to_string()))?;

        Ok(Session {
            token,
            user_id,
            expires_at: expires_at_str,
        })
    }

    fn validate(&self, token: &str) -> Result<Session, AuthError> {
        let conn = self
            .get()
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        let session = conn
            .query_row(
                "SELECT user_id, expires_at FROM _brom_session WHERE token = ?1",
                [token],
                |row| {
                    Ok(Session {
                        token: token.to_string(),
                        user_id: row.get(0)?,
                        expires_at: row.get(1)?,
                    })
                },
            )
            .map_err(|_| AuthError::InvalidSession)?;

        // Check expiry
        let expires_at = DateTime::parse_from_rfc3339(&session.expires_at)
            .map_err(|_| AuthError::InternalError("Invalid timestamp in DB".into()))?
            .with_timezone(&Utc);

        if Utc::now() > expires_at {
            return Err(AuthError::InvalidSession);
        }

        Ok(session)
    }

    fn destroy(&self, token: &str) -> Result<(), AuthError> {
        let conn = self
            .get()
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        conn.execute("DELETE FROM _brom_session WHERE token = ?1", [token])
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        Ok(())
    }

    fn cleanup_expired(&self) -> Result<u64, AuthError> {
        let conn = self
            .get()
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        let now = Utc::now().to_rfc3339();
        let deleted = conn
            .execute("DELETE FROM _brom_session WHERE expires_at < ?1", [now])
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        Ok(deleted as u64)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::migration::MigrationRunner;
    use crate::pool::DbPool;

    fn setup_test_db() -> DbPool {
        // Use a unique shared in-memory database name per test for isolation
        let db_name = format!(
            "file:memdb_{}?mode=memory&cache=shared",
            rand::random::<u32>()
        );
        let pool = DbPool::new(&db_name).expect("Failed to create in-memory DB");
        let runner = MigrationRunner::new(&pool);
        runner
            .ensure_internal_tables()
            .expect("Failed to run internal migrations");

        // Need a user for FK constraints
        // narsil-ignore: RUST-002
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO _brom_user (email, password_hash, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            (format!("test_{}@example.com", rand::random::<u32>()), "hash", "2026-01-01T00:00:00Z", "2026-01-01T00:00:00Z"),
        ).unwrap();

        pool
    }

    #[test]
    fn test_session_lifecycle() {
        let pool = setup_test_db();

        // Create session
        let session = pool.create(1).expect("Failed to create session");
        assert_eq!(session.user_id, 1);
        assert_eq!(session.token.len(), 64); // 32 bytes hex encoded

        // Validate session
        let validated = pool
            .validate(&session.token)
            .expect("Failed to validate session");
        assert_eq!(validated.user_id, 1);

        // Destroy session
        pool.destroy(&session.token)
            .expect("Failed to destroy session");

        // Validate should now fail
        let result = pool.validate(&session.token);
        assert!(matches!(result, Err(AuthError::InvalidSession)));
    }

    #[test]
    fn test_session_expiry() {
        let pool = setup_test_db();
        let conn = pool.get().unwrap();

        // Insert expired session manually
        let expired_token = "expired-token";
        let past = Utc::now() - Duration::hours(1);
        conn.execute(
            "INSERT INTO _brom_session (user_id, token, expires_at) VALUES (?1, ?2, ?3)",
            (1, expired_token, &past.to_rfc3339()),
        )
        .unwrap();

        // Validate should fail
        let result = pool.validate(expired_token);
        assert!(matches!(result, Err(AuthError::InvalidSession)));

        // Cleanup should remove it
        let cleaned = pool.cleanup_expired().expect("Failed cleanup");
        assert_eq!(cleaned, 1);
    }
}
