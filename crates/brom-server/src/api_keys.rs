use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use brom_auth::{ApiKeyRecord, Permission};
use serde::{Deserialize, Serialize};

use crate::{error::ServerError, extractor::RequireAdmin, state::AppState};

/// Data Transfer Object representing an API key without the raw secret.
#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ApiKeyRecordDto {
    /// Unique identifier for the API key.
    pub id: i64,
    /// Human-readable label for the key.
    pub name: String,
    /// Hint containing the first few characters of the key identifier.
    pub key_prefix: String,
    /// Access control level or capability tags.
    pub permissions: String,
    /// ID of the user who owns the key.
    pub user_id: i64,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
    /// ISO 8601 timestamp of last usage, if ever used.
    pub last_used_at: Option<String>,
}

impl From<ApiKeyRecord> for ApiKeyRecordDto {
    fn from(record: ApiKeyRecord) -> Self {
        Self {
            id: record.id,
            name: record.name,
            key_prefix: record.key_prefix,
            permissions: record.permissions.to_string(),
            user_id: record.user_id,
            created_at: record.created_at,
            last_used_at: record.last_used_at,
        }
    }
}

/// Payload for provisioning a new API key.
#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateApiKeyRequest {
    /// Human-readable label for the key.
    pub name: String,
    /// Access control level or capability tags.
    pub permissions: String,
}

/// Response containing the newly generated API key secret.
#[derive(Serialize, utoipa::ToSchema)]
pub struct CreateApiKeyResponse {
    /// Plaint-text secret token - only shown once upon creation.
    pub raw_key: String,
    /// Redacted record properties of the newly created key.
    pub record: ApiKeyRecordDto,
}

#[utoipa::path(
    get,
    path = "/admin/api/keys",
    responses(
        (status = 200, description = "List API keys", body = [ApiKeyRecordDto]),
        (status = 401, description = "Unauthorized")
    ),
    tag = "admin",
    security(("session" = []))
)]
/// Lists API keys for the current user.
///
/// # Errors
/// Returns `ServerError` if a database error occurs.
#[tracing::instrument(skip_all)]
pub async fn list_keys(
    RequireAdmin(session): RequireAdmin,
    State(state): State<AppState>,
) -> Result<Json<Vec<ApiKeyRecordDto>>, ServerError> {
    let keys = state.api_key_store.list_for_user(session.user_id)?;
    let dtos = keys.into_iter().map(Into::into).collect();
    Ok(Json(dtos))
}

#[utoipa::path(
    post,
    path = "/admin/api/keys",
    request_body = CreateApiKeyRequest,
    responses(
        (status = 201, description = "Create API key", body = CreateApiKeyResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "admin",
    security(("session" = []))
)]
/// Creates a new API key.
///
/// # Errors
/// Returns `ServerError` if a database error occurs or creation fails.
#[tracing::instrument(skip_all)]
pub async fn create_key(
    RequireAdmin(session): RequireAdmin,
    State(state): State<AppState>,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<CreateApiKeyResponse>), ServerError> {
    let permissions = payload.permissions.parse::<Permission>()?;

    let (raw_key, record) = state
        .api_key_store
        .create(session.user_id, &payload.name, permissions)?;
    Ok((
        StatusCode::CREATED,
        Json(CreateApiKeyResponse {
            raw_key,
            record: record.into(),
        }),
    ))
}

#[utoipa::path(
    delete,
    path = "/admin/api/keys/{id}",
    params(("id" = i64, Path, description = "API Key ID")),
    responses(
        (status = 204, description = "Revoke API key"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "admin",
    security(("session" = []))
)]
/// Revokes an API key.
///
/// # Errors
/// Returns `ServerError` if a database error occurs.
#[tracing::instrument(skip_all)]
pub async fn revoke_key(
    RequireAdmin(session): RequireAdmin,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ServerError> {
    state.api_key_store.revoke(id, session.user_id)?;
    Ok(StatusCode::NO_CONTENT)
}
