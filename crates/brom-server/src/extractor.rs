use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts},
};
use brom_auth::{ApiKeyRecord, Session};
use crate::{state::AppState, error::ServerError};

/// Extractor that requires a valid admin session cookie.
pub struct RequireAdmin(pub Session);

impl<S> FromRequestParts<S> for RequireAdmin
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        let cookie_header = parts
            .headers
            .get(header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .ok_or(brom_auth::AuthError::InvalidSession)?;

        // Basic cookie parsing for "brom_session=<token>"
        let token = cookie_header
            .split(';')
            .map(|s| s.trim())
            .find(|s| s.starts_with("brom_session="))
            .and_then(|s| s.strip_prefix("brom_session="))
            .ok_or(brom_auth::AuthError::InvalidSession)?;

        let session = state.session_store.validate(token)?;
        Ok(RequireAdmin(session))
    }
}

/// Extractor that requires a valid API key in the Authorization header.
pub struct RequireApiKey(pub ApiKeyRecord);

impl<S> FromRequestParts<S> for RequireApiKey
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(brom_auth::AuthError::InvalidApiKey)?;

        if !auth_header.to_lowercase().starts_with("bearer ") {
            return Err(brom_auth::AuthError::InvalidApiKey.into());
        }

        let key = auth_header[7..].trim();
        let record = state.api_key_store.validate(key)?;
        Ok(RequireApiKey(record))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use axum::http::Request;
    use brom_auth::{MockSessionStore, MockApiKeyStore};
    use brom_db::DbPool;

    fn test_state(
        session_store: Arc<dyn brom_auth::SessionStore>,
        api_key_store: Arc<dyn brom_auth::ApiKeyStore>,
    ) -> AppState {
        AppState {
            db: DbPool::in_memory().unwrap(), // Mock DB
            session_store,
            api_key_store,
            schema_registry: Arc::new(brom_core::schema::SchemaRegistry::new()),
        }
    }

    #[tokio::test]
    async fn test_require_admin_valid_session() {
        let mut mock_sessions = MockSessionStore::new();
        let expected_session = Session {
            token: "valid_token".into(),
            user_id: 1,
            expires_at: "never".into(),
        };
        let session_clone = expected_session.clone();
        mock_sessions
            .expect_validate()
            .with(mockall::predicate::eq("valid_token"))
            .returning(move |_| Ok(session_clone.clone()));

        let state = test_state(Arc::new(mock_sessions), Arc::new(MockApiKeyStore::new()));
        
        let mut request = Request::builder()
            .header("Cookie", "brom_session=valid_token")
            .body(())
            .unwrap();
        let (mut parts, _) = request.into_parts();

        let result = RequireAdmin::from_request_parts(&mut parts, &state).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.user_id, 1);
    }

    #[tokio::test]
    async fn test_require_api_key_valid() {
        let mut mock_keys = MockApiKeyStore::new();
        let expected_record = ApiKeyRecord {
            id: 1,
            name: "test".into(),
            key_prefix: "br_".into(),
            permissions: "read".into(),
            user_id: 1,
            created_at: "now".into(),
            last_used_at: None,
        };
        let record_clone = expected_record.clone();
        mock_keys
            .expect_validate()
            .with(mockall::predicate::eq("secret_key"))
            .returning(move |_| Ok(record_clone.clone()));

        let state = test_state(Arc::new(MockSessionStore::new()), Arc::new(mock_keys));

        let mut request = Request::builder()
            .header("Authorization", "Bearer secret_key")
            .body(())
            .unwrap();
        let (mut parts, _) = request.into_parts();

        let result = RequireApiKey::from_request_parts(&mut parts, &state).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0.id, 1);
    }
}
