use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

/// Global top-level server error, wrapping nested domain errors.
#[derive(Debug, Error)]
pub enum ServerError {
    /// Domain logic and validation errors.
    #[error(transparent)]
    Core(#[from] brom_core::Error),

    /// Infrastructure and persistence layer errors.
    #[error(transparent)]
    Db(#[from] brom_db::DbError),

    /// Security, credential, and authentication errors.
    #[error(transparent)]
    Auth(#[from] brom_auth::AuthError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, ident, message) = match &self {
            ServerError::Auth(
                brom_auth::AuthError::InvalidSession
                | brom_auth::AuthError::InvalidCredentials
                | brom_auth::AuthError::InvalidApiKey,
            ) => (StatusCode::UNAUTHORIZED, "Unauthorized", self.to_string()),
            ServerError::Auth(brom_auth::AuthError::InsufficientPermissions(_)) => {
                (StatusCode::FORBIDDEN, "Forbidden", self.to_string())
            }
            ServerError::Core(brom_core::Error::NotFound { .. }) => {
                (StatusCode::NOT_FOUND, "NotFound", self.to_string())
            }
            ServerError::Core(brom_core::Error::ValidationError { .. }) => (
                StatusCode::BAD_REQUEST,
                "ValidationFailed",
                self.to_string(),
            ),
            ServerError::Core(brom_core::Error::UniqueViolation { .. }) => {
                (StatusCode::CONFLICT, "UniqueViolation", self.to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "InternalError",
                "Internal server error".to_string(),
            ),
        };

        let mut body = serde_json::Map::new();
        body.insert("error".to_string(), json!(ident));
        body.insert("message".to_string(), json!(message));

        if let ServerError::Core(brom_core::Error::ValidationError { field, .. }) = &self {
            body.insert("fields".to_string(), json!({ field: [message] }));
        }

        (status, Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
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

    #[test]
    fn unique_violation_returns_409() {
        assert_eq!(
            status_of(ServerError::Core(brom_core::Error::UniqueViolation {
                entity: "user",
                field: "email".into(),
                value: "taken@example.com".into(),
            })),
            StatusCode::CONFLICT,
        );
    }

    #[tokio::test]
    async fn validation_error_body_format() {
        let err = ServerError::Core(brom_core::Error::ValidationError {
            field: "title".into(),
            message: "validation msg".into(),
        });

        let mut response = err.into_response();
        // narsil-ignore: RUST-002
        let body_bytes = http_body_util::BodyExt::collect(response.body_mut())
            .await
            .unwrap()
            .to_bytes();
        // narsil-ignore: RUST-002
        let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(body["error"], "ValidationFailed");
        assert_eq!(
            body["message"],
            "validation error for field 'title': validation msg"
        );
        assert_eq!(
            body["fields"]["title"][0],
            "validation error for field 'title': validation msg"
        );
    }
}
