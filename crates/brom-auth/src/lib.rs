//! Authentication and authorization for the brom headless CMS framework.

pub mod api_key;
pub mod error;
pub mod password;
pub mod rbac;
pub mod session;
pub use api_key::{ApiKeyRecord, ApiKeyStore};
pub use error::AuthError;
pub use rbac::evaluate_policy;
pub use session::{Session, SessionStore};

#[cfg(feature = "testing")]
pub use api_key::MockApiKeyStore;
#[cfg(feature = "testing")]
pub use session::MockSessionStore;
