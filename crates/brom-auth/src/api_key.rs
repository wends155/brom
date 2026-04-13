use std::fmt;
use std::str::FromStr;

use crate::error::AuthError;
use serde::{Deserialize, Serialize};

/// Access control level for API keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    /// Read-only access to entity endpoints.
    Read,
    /// Full read and write access to entity endpoints.
    ReadWrite,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => write!(f, "read"),
            Self::ReadWrite => write!(f, "read_write"),
        }
    }
}

impl FromStr for Permission {
    type Err = AuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "read" => Ok(Self::Read),
            "read_write" => Ok(Self::ReadWrite),
            other => Err(AuthError::InternalError(format!(
                "invalid permission value: {other}"
            ))),
        }
    }
}

/// Stored metadata for an API key.
/// The raw key is never persisted; only its hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRecord {
    pub id: i64,
    pub name: String,
    pub key_prefix: String,
    pub permissions: Permission,
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
        permissions: Permission,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_permission_display_and_parse() {
        // Display
        assert_eq!(Permission::Read.to_string(), "read");
        assert_eq!(Permission::ReadWrite.to_string(), "read_write");

        // FromStr
        assert_eq!(Permission::from_str("read").unwrap(), Permission::Read);
        assert_eq!(Permission::from_str("read_write").unwrap(), Permission::ReadWrite);

        // Invalid
        assert!(Permission::from_str("admin").is_err());
        assert!(Permission::from_str("").is_err());
        assert!(Permission::from_str("READ").is_err());
    }

    #[test]
    fn test_permission_serde_roundtrip() {
        let read = Permission::Read;
        let json = serde_json::to_string(&read).unwrap();
        assert_eq!(json, "\"read\"");

        let deserialized: Permission = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Permission::Read);

        let rw = Permission::ReadWrite;
        let json = serde_json::to_string(&rw).unwrap();
        assert_eq!(json, "\"read_write\"");
    }
}
