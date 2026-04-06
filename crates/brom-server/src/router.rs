use axum::response::IntoResponse;
use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, header},
    routing::{get, post},
};
use serde::Deserialize;
use serde::Serialize;
use tower_http::trace::TraceLayer;

use crate::{
    error::ServerError, extractor::RequireAdmin, middleware, openapi, schema_api, state::AppState,
};
use brom_auth::password::verify_password;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct LoginResponse {
    pub message: String,
    pub user_id: i64,
}

/// Handler for `POST /admin/api/login`.
/// Verifies credentials and sets a session cookie.
///
/// # Errors
/// Returns `ServerError` if database query fails or credentials are invalid.
#[utoipa::path(
    post,
    path = "/admin/api/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "admin"
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, ServerError> {
    let conn = state.db.get()?;

    let (user_id, password_hash): (i64, String) = conn
        .query_row(
            "SELECT id, password_hash FROM _brom_user WHERE email = ?1",
            [&payload.email],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|_| brom_auth::AuthError::InvalidCredentials)?;

    verify_password(&payload.password, &password_hash)?;

    let session = state.session_store.create(user_id)?;

    let cookie = format!(
        "brom_session={}; HttpOnly; Path=/; SameSite=Lax; Max-Age=86400",
        session.token
    );

    let body = Json(LoginResponse {
        message: "Login successful".into(),
        user_id,
    });

    Ok((StatusCode::OK, [(header::SET_COOKIE, cookie)], body))
}

/// Handler for `POST /admin/api/logout`.
/// Destroys the current session and clears the cookie.
///
/// # Errors
/// Returns `ServerError` if the session could not be destroyed in the store.
#[utoipa::path(
    post,
    path = "/admin/api/logout",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Invalid session")
    ),
    tag = "admin",
    security(
        ("session" = [])
    )
)]
pub async fn logout(
    RequireAdmin(session): RequireAdmin,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ServerError> {
    state.session_store.destroy(&session.token)?;

    let cookie = "brom_session=; HttpOnly; Path=/; SameSite=Lax; Max-Age=0";

    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        "Logout successful",
    ))
}

/// Builds the complete Axum router for the brom-server.
pub fn build_router(state: AppState, cors_origins: Vec<axum::http::HeaderValue>) -> Router {
    Router::new()
        .route("/admin/api/login", post(login))
        .route("/admin/api/logout", post(logout))
        .route("/admin/api/schema", get(schema_api::get_schema))
        .merge(openapi::swagger_ui())
        .layer(middleware::cors_layer(cors_origins))
        .layer(middleware::security_headers_layer())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
