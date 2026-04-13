use crate::error::AuthError;
use serde::{Deserialize, Serialize};

/// Represents an active admin session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// The unique session token (usually a secure random string).
    pub token: String,
    /// The ID of the authenticated user.
    pub user_id: i64,
    /// The expiration timestamp in ISO 8601 format.
    pub expires_at: String,
}

/// Manages the lifecycle of admin UI sessions.
#[cfg_attr(feature = "testing", mockall::automock)]
pub trait SessionStore: Send + Sync {
    /// Creates a new session for the given user.
    ///
    /// # Errors
    /// Returns `AuthError::InternalError` if session creation fails.
    fn create(&self, user_id: i64) -> Result<Session, AuthError>;

    /// Validates a session token.
    ///
    /// # Errors
    /// Returns `AuthError::InvalidSession` if the token is not found or has expired.
    fn validate(&self, token: &str) -> Result<Session, AuthError>;

    /// Destroys a session (logout).
    ///
    /// # Errors
    /// Returns `AuthError::InternalError` if the operation fails.
    fn destroy(&self, token: &str) -> Result<(), AuthError>;

    /// Removes all expired sessions from the store.
    /// Returns the number of sessions removed.
    ///
    /// # Errors
    /// Returns `AuthError::InternalError` if cleanup fails.
    fn cleanup_expired(&self) -> Result<u64, AuthError>;

    /// Destroys all sessions for a specific user (Mass Invalidation).
    ///
    /// # Errors
    /// Returns `AuthError::InternalError` if the operation fails.
    fn destroy_all_for_user(&self, user_id: i64) -> Result<(), AuthError>;
}
