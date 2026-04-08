//! Authentication and authorization for the `brom` headless CMS framework.

/// API key record definitions and retrieval.
pub mod api_key;
/// Authentication specific error types.
pub mod error;
/// Password hashing and verification utilities.
pub mod password;
/// Role-based access control evaluation.
pub mod rbac;
/// Session management and persistent storage.
pub mod session;
pub use api_key::{ApiKeyRecord, ApiKeyStore};
pub use error::AuthError;
pub use rbac::evaluate_policy;
pub use session::{Session, SessionStore};

#[cfg(feature = "testing")]
pub use api_key::MockApiKeyStore;
#[cfg(feature = "testing")]
pub use session::MockSessionStore;
