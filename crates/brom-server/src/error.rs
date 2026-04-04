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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    fn status_of(err: ServerError) -> StatusCode {
        err.into_response().status()
    }

    #[test]
    fn invalid_credentials_returns_401() {
        assert_eq!(
            status_of(ServerError::Auth(brom_auth::AuthError::InvalidCredentials)),
            StatusCode::UNAUTHORIZED,
        );
    }

    #[test]
    fn invalid_session_returns_401() {
        assert_eq!(
            status_of(ServerError::Auth(brom_auth::AuthError::InvalidSession)),
            StatusCode::UNAUTHORIZED,
        );
    }

    #[test]
    fn invalid_api_key_returns_401() {
        assert_eq!(
            status_of(ServerError::Auth(brom_auth::AuthError::InvalidApiKey)),
            StatusCode::UNAUTHORIZED,
        );
    }

    #[test]
    fn insufficient_permissions_returns_403() {
        assert_eq!(
            status_of(ServerError::Auth(
                brom_auth::AuthError::InsufficientPermissions("write".into()),
            )),
            StatusCode::FORBIDDEN,
        );
    }

    #[test]
    fn not_found_returns_404() {
        assert_eq!(
            status_of(ServerError::Core(brom_core::Error::NotFound {
                entity: "post",
                id: 42,
            })),
            StatusCode::NOT_FOUND,
        );
    }

    #[test]
    fn validation_error_returns_400() {
        assert_eq!(
            status_of(ServerError::Core(brom_core::Error::ValidationError {
                field: "title".into(),
                message: "required".into(),
            })),
            StatusCode::BAD_REQUEST,
        );
    }

    #[test]
    fn generic_core_error_returns_500() {
        assert_eq!(
            status_of(ServerError::Core(brom_core::Error::SchemaError(
                "unknown".into(),
            ))),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
    }
}
