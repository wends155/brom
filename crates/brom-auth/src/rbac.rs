use crate::api_key::ApiKeyStore;
use crate::error::AuthError;
use crate::session::SessionStore;
use brom_core::AuthPolicy;

/// Evaluates if a given token or session meets the requirements of an `AuthPolicy`.
///
/// # Errors
/// - `AuthError::InvalidCredentials` if no token is provided for protected routes.
/// - `AuthError::InvalidSession` if session validation fails.
/// - `AuthError::InvalidApiKey` if api key validation fails.
/// - `AuthError::InsufficientPermissions` if the policy requirements (e.g. `read_write`) are not met.
pub fn evaluate_policy(
    policy: &AuthPolicy,
    token: Option<&str>,
    session_store: &dyn SessionStore,
    api_key_store: &dyn ApiKeyStore,
) -> Result<(), AuthError> {
    match policy {
        AuthPolicy::Public => Ok(()),

        AuthPolicy::AdminOnly => {
            let token = token.ok_or(AuthError::InvalidCredentials)?;
            // For AdminOnly, we check the session store
            session_store.validate(token).map(|_| ())
        }

        AuthPolicy::ApiKey => {
            let token = token.ok_or(AuthError::InvalidCredentials)?;
            // For ApiKey, we check the api key store
            api_key_store.validate(token).map(|_| ())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_key::MockApiKeyStore;
    use crate::session::MockSessionStore;
    use crate::session::Session;

    #[test]
    fn test_evaluate_policy_public() {
        let session_store = MockSessionStore::new();
        let api_key_store = MockApiKeyStore::new();

        let result = evaluate_policy(&AuthPolicy::Public, None, &session_store, &api_key_store);
        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_policy_admin_only() {
        let mut session_store = MockSessionStore::new();
        let api_key_store = MockApiKeyStore::new();

        // Success case
        session_store
            .expect_validate()
            .with(mockall::predicate::eq("valid-session"))
            .returning(|_| {
                Ok(Session {
                    token: "valid-session".into(),
                    user_id: 1,
                    expires_at: "2026-04-04T12:00:00Z".into(),
                })
            });

        let result = evaluate_policy(
            &AuthPolicy::AdminOnly,
            Some("valid-session"),
            &session_store,
            &api_key_store,
        );
        assert!(result.is_ok());

        // Failure case: missing token
        let result = evaluate_policy(&AuthPolicy::AdminOnly, None, &session_store, &api_key_store);
        assert!(matches!(result, Err(AuthError::InvalidCredentials)));

        // Failure case: invalid token
        let mut session_store = MockSessionStore::new();
        session_store
            .expect_validate()
            .returning(|_| Err(AuthError::InvalidSession));
        let result = evaluate_policy(
            &AuthPolicy::AdminOnly,
            Some("invalid"),
            &session_store,
            &api_key_store,
        );
        assert!(matches!(result, Err(AuthError::InvalidSession)));
    }
}
