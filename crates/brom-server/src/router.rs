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

/// Payload for generating a new session cookie.
#[derive(Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    /// Account email address.
    pub email: String,
    /// Plaintext password to verify against the stored hash.
    pub password: String,
}

/// Result of a successful login attempt.
#[derive(Serialize, utoipa::ToSchema)]
pub struct LoginResponse {
    /// Status message or confirmation text.
    pub message: String,
    /// Internal ID of the authenticated user.
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
#[tracing::instrument(skip_all)]
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
#[tracing::instrument(skip_all)]
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
        .route(
            "/admin/api/keys",
            get(crate::api_keys::list_keys).post(crate::api_keys::create_key),
        )
        .route(
            "/admin/api/keys/{id}",
            axum::routing::delete(crate::api_keys::revoke_key),
        )
        .merge(openapi::swagger_ui())
        .layer(middleware::cors_layer(cors_origins))
        .layer(middleware::x_content_type_options_layer())
        .layer(middleware::x_frame_options_layer())
        .layer(middleware::referrer_policy_layer())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        tracing::info!(
                            latency = ?latency,
                            status = %response.status(),
                            "finished processing request"
                        );
                    },
                ),
        )
        .with_state(state)
}
