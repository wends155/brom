use crate::state::AppState;
use axum::{Json, extract::State};
use brom_core::entity::SchemaInfo;

/// Handler for `GET /admin/api/schema`.
/// Returns all registered entity schemas.
///
/// This endpoint is used by the admin UI to discover available content types
/// and their field structures for dynamic form generation.
#[tracing::instrument(skip_all)]
pub async fn get_schema(State(state): State<AppState>) -> Json<Vec<SchemaInfo>> {
    Json(state.schema_registry.all_schemas())
}
