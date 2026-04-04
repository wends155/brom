//! Authentication and authorization for the brom headless CMS framework.

pub mod error;
pub use error::AuthError;

use brom_core::AuthPolicy;

/// Trait to manage admin UI sessions.
#[cfg_attr(feature = "testing", mockall::automock)]
pub trait SessionStore: Send + Sync {
    // STUB(Phase 3): Define session management methods
}

/// Trait to manage API Keys.
#[cfg_attr(feature = "testing", mockall::automock)]
pub trait ApiKeyStore: Send + Sync {
    // STUB(Phase 3): Define API key methods
}

/// Generic policy evaluator
///
/// # Errors
/// Returns `AuthError` if the policy requirement is not met by the token/session.
#[tracing::instrument(skip(_token_or_session), fields(policy = ?policy))]
pub fn evaluate_policy(
    policy: &AuthPolicy,
    _token_or_session: Option<&str>,
) -> Result<(), AuthError> {
    // STUB(Phase 3): Implement evaluation logic
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use brom_core::AuthPolicy;

    #[test]
    fn evaluate_policy_public_succeeds() {
        let result = evaluate_policy(&AuthPolicy::Public, None);
        assert!(result.is_ok());
    }

    #[test]
    fn evaluate_policy_api_key_succeeds_stub() {
        // Stub currently returns Ok for all policies
        let result = evaluate_policy(&AuthPolicy::ApiKey, Some("test-token"));
        assert!(result.is_ok());
    }

    #[test]
    fn evaluate_policy_admin_only_succeeds_stub() {
        // Stub currently returns Ok for all policies
        let result = evaluate_policy(&AuthPolicy::AdminOnly, None);
        assert!(result.is_ok());
    }
}
