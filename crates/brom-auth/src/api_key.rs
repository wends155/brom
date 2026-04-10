use crate::error::AuthError;
use serde::{Deserialize, Serialize};

/// Stored metadata for an API key.
/// The raw key is never persisted; only its hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRecord {
    pub id: i64,
    pub name: String,
    pub key_prefix: String,
    pub permissions: String, // "read" or "read_write"
    pub user_id: i64,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

/// Manages the lifecycle and validation of API keys.
#[cfg_attr(feature = "testing", mockall::automock)]
pub trait ApiKeyStore: Send + Sync {
    /// Creates a new API key for a user.
    /// Returns the raw key string (only shown once) and the record.
    ///
    /// # Errors
    /// Returns `AuthError::InternalError` if creation fails.
    fn create(
        &self,
        user_id: i64,
        name: &str,
        permissions: &str,
    ) -> Result<(String, ApiKeyRecord), AuthError>;

    /// Validates a raw API key.
    /// Returns the associated record if valid.
    ///
    /// # Errors
    /// Returns `AuthError::InvalidApiKey` if the key is invalid or revoked.
    fn validate(&self, raw_key: &str) -> Result<ApiKeyRecord, AuthError>;

    /// Revokes an API key by ID, scoped to the owning user.
    ///
    /// # Errors
    /// Returns `AuthError::InternalError` if the operation fails.
    /// Returns `AuthError::InvalidApiKey` if the key does not belong to the user.
    fn revoke(&self, id: i64, user_id: i64) -> Result<(), AuthError>;

    /// Lists all API keys for a specific user.
    ///
    /// # Errors
    /// Returns `AuthError::InternalError` if lookup fails.
    fn list_for_user(&self, user_id: i64) -> Result<Vec<ApiKeyRecord>, AuthError>;
}
