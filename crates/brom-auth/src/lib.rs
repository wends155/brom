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
#[tracing::instrument(skip_all)]
pub fn evaluate_policy(
    _policy: &AuthPolicy,
    _token_or_session: Option<&str>,
) -> Result<(), AuthError> {
    // STUB(Phase 3): Implement evaluation logic
    Ok(())
}
