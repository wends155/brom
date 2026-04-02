use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    Core(#[from] brom_core::Error),

    #[error(transparent)]
    Db(#[from] brom_db::DbError),

    #[error(transparent)]
    Auth(#[from] brom_auth::AuthError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            ServerError::Auth(
                brom_auth::AuthError::InvalidSession
                | brom_auth::AuthError::InvalidCredentials
                | brom_auth::AuthError::InvalidApiKey,
            ) => (StatusCode::UNAUTHORIZED, self.to_string()),
            ServerError::Auth(brom_auth::AuthError::InsufficientPermissions(_)) => {
                (StatusCode::FORBIDDEN, self.to_string())
            }
            ServerError::Core(brom_core::Error::NotFound { .. }) => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            ServerError::Core(brom_core::Error::ValidationError { .. }) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
