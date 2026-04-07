#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

use brom_macros::BromEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, BromEntity, utoipa::ToSchema)]
#[brom(table = "_test_public_posts", auth_policy = "None")]
pub struct PublicPost {
    pub id: i64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, BromEntity, utoipa::ToSchema)]
#[brom(table = "_test_api_keys_settings", auth_policy = "ApiKey")]
pub struct ApiKeySetting {
    pub id: i64,
    pub config_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, BromEntity, utoipa::ToSchema)]
#[brom(table = "_test_admin_only", auth_policy = "AdminOnly")]
pub struct AdminOnlyData {
    pub id: i64,
    pub secret_val: String,
}

/// Helper function to stand up the mock schema
fn setup_mock_tables(state: &brom_server::AppState) {
    let conn = state.db.get().unwrap();
    conn.execute_batch(
        "
        CREATE TABLE _test_public_posts (id INTEGER PRIMARY KEY, title TEXT);
        INSERT INTO _test_public_posts (title) VALUES ('Hello Public');

        CREATE TABLE _test_api_keys_settings (id INTEGER PRIMARY KEY, config_key TEXT);
        INSERT INTO _test_api_keys_settings (config_key) VALUES ('Api Key Config');

        CREATE TABLE _test_admin_only (id INTEGER PRIMARY KEY, secret_val TEXT);
        INSERT INTO _test_admin_only (secret_val) VALUES ('Secret Admin Data');
        ",
    )
    .unwrap();
}

#[tokio::test]
async fn test_auth_policy_none() {
    let state = common::test_app_state();
    setup_mock_tables(&state);

    // We only mount the public router for this entity
    let app = PublicPost::public_router().with_state(state.clone());

    let request = Request::builder()
        .uri("/api/v1/entities/publicpost")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body["data"][0]["title"], "Hello Public");
}

#[tokio::test]
async fn test_auth_policy_api_key() {
    use brom_auth::ApiKeyStore;
    let state = common::test_app_state();
    setup_mock_tables(&state);

    // Mock key creation in DB
    let pool = state.db.clone();
    let conn = pool.get().unwrap();
    conn.execute(
        "INSERT INTO _brom_user (email, password_hash, created_at, updated_at) VALUES ('user@test.com', 'hash', '2026-01-01', '2026-01-01')",
        ()
    ).unwrap();
    let user_id = conn.last_insert_rowid();
    let (raw_key, _) = pool.create(user_id, "test_key", "read").unwrap();

    let app = ApiKeySetting::public_router().with_state(state.clone());

    // 1. Without API key -> 401
    let req_unauth = Request::builder()
        .uri("/api/v1/entities/apikeysetting")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(req_unauth).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 2. With API key -> 200
    let req_auth = Request::builder()
        .uri("/api/v1/entities/apikeysetting")
        .header(header::AUTHORIZATION, format!("Bearer {raw_key}"))
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(req_auth).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body["data"][0]["config_key"], "Api Key Config");
}

#[tokio::test]
async fn test_auth_policy_admin_only() {
    let state = common::test_app_state();
    setup_mock_tables(&state);

    // AdminOnly compiles to `Router::new()` which is empty, so calling it
    // without `.fallback()` on Axum will result in a 404 NOT FOUND from Axum for the route
    let app = AdminOnlyData::public_router().with_state(state.clone());

    let request = Request::builder()
        .uri("/api/v1/entities/adminonlydata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // Since Router::new() does not register the path, we expect a 404 NOT FOUND.
    // That means the policy generation correctly omits the route.
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
