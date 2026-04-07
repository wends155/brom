use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use brom_auth::ApiKeyRecord;
use serde::{Deserialize, Serialize};

use crate::{error::ServerError, extractor::RequireAdmin, state::AppState};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ApiKeyRecordDto {
    pub id: i64,
    pub name: String,
    pub key_prefix: String,
    pub permissions: String,
    pub user_id: i64,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

impl From<ApiKeyRecord> for ApiKeyRecordDto {
    fn from(record: ApiKeyRecord) -> Self {
        Self {
            id: record.id,
            name: record.name,
            key_prefix: record.key_prefix,
            permissions: record.permissions,
            user_id: record.user_id,
            created_at: record.created_at,
            last_used_at: record.last_used_at,
        }
    }
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub permissions: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct CreateApiKeyResponse {
    pub raw_key: String,
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
pub async fn create_key(
    RequireAdmin(session): RequireAdmin,
    State(state): State<AppState>,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<CreateApiKeyResponse>), ServerError> {
    let (raw_key, record) =
        state
            .api_key_store
            .create(session.user_id, &payload.name, &payload.permissions)?;
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
pub async fn revoke_key(
    RequireAdmin(_session): RequireAdmin,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ServerError> {
    state.api_key_store.revoke(id)?;
    Ok(StatusCode::NO_CONTENT)
}
