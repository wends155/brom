use axum::{extract::State, Json};
use brom_core::entity::SchemaInfo;
use crate::state::AppState;

/// Handler for `GET /admin/api/schema`.
/// Returns all registered entity schemas.
/// 
/// This endpoint is used by the admin UI to discover available content types
/// and their field structures for dynamic form generation.
pub async fn get_schema(
    State(state): State<AppState>,
) -> Json<Vec<SchemaInfo>> {
    Json(state.schema_registry.all_schemas())
}
