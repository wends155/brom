use thiserror::Error;

/// Authentication errors for the brom framework.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("session expired or invalid")]
    InvalidSession,

    #[error("invalid api key")]
    InvalidApiKey,

    #[error("insufficient permissions: {0}")]
    InsufficientPermissions(String),

    #[error("internal auth error: {0}")]
    InternalError(String),
}
