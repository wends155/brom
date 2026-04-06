#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt; // for oneshot

use brom_server::router::build_router;

/// Helper: send a request through the full router and return (status, body_json).
async fn send_json(
    state: brom_server::AppState,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let app = build_router(state, vec![]);

    let mut builder = Request::builder().uri(uri).method(method);

    let body = if let Some(json_body) = body {
        builder = builder.header(header::CONTENT_TYPE, "application/json");
        Body::from(serde_json::to_vec(&json_body).unwrap())
    } else {
        Body::empty()
    };

    let request = builder.body(body).unwrap();
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap_or(Value::Null);
    (status, json)
}

#[tokio::test]
async fn login_valid_credentials_returns_200_and_cookie() {
    let state = common::test_app_state();
    let (_user_id, password) = common::seed_admin_user(&state);
    let origins = vec![header::HeaderValue::from_static("http://localhost:3000")];

    let app = build_router(state, origins);
    let request = Request::builder()
        .method("POST")
        .uri("/admin/api/login")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "email": "admin@test.com",
                "password": password
            }))
            .unwrap(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify Set-Cookie header is present
    let cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .expect("Set-Cookie header missing");
    let cookie_str = cookie.to_str().unwrap();
    assert!(cookie_str.starts_with("brom_session="));
    assert!(cookie_str.contains("HttpOnly"));

    // Verify response body
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body["message"], "Login successful");
}

#[tokio::test]
async fn login_invalid_password_returns_401() {
    let state = common::test_app_state();
    common::seed_admin_user(&state);

    let (status, body) = send_json(
        state,
        "POST",
        "/admin/api/login",
        Some(json!({
            "email": "admin@test.com",
            "password": "wrong_password"
        })),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let err_str = body["error"].as_str().expect("body missing error");
    assert!(
        err_str.to_lowercase().contains("invalid"),
        "Expected 'invalid' but got: {err_str}"
    );
}

#[tokio::test]
async fn login_nonexistent_user_returns_401() {
    let state = common::test_app_state();

    let (status, _body) = send_json(
        state,
        "POST",
        "/admin/api/login",
        Some(json!({
            "email": "nobody@test.com",
            "password": "anything"
        })),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn logout_with_valid_session_returns_200() {
    let state = common::test_app_state();
    let (user_id, _password) = common::seed_admin_user(&state);
    let token = common::create_test_session(&state, user_id);

    let app = build_router(state, vec![]);
    let request = Request::builder()
        .method("POST")
        .uri("/admin/api/logout")
        .header(header::COOKIE, format!("brom_session={token}"))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Verify cookie is cleared (Max-Age=0)
    let cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .expect("Set-Cookie header missing");
    let cookie_str = cookie.to_str().unwrap();
    assert!(cookie_str.contains("Max-Age=0"));
}

#[tokio::test]
async fn logout_without_session_returns_401() {
    let state = common::test_app_state();

    let app = build_router(state, vec![]);
    let request = Request::builder()
        .method("POST")
        .uri("/admin/api/logout")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_schema_returns_empty_array() {
    let state = common::test_app_state();

    let app = build_router(state, vec![]);
    let request = Request::builder()
        .uri("/admin/api/schema")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn security_headers_are_present() {
    let state = common::test_app_state();

    let app = build_router(state, vec![]);
    let request = Request::builder()
        .uri("/admin/api/schema")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    let xcto = response
        .headers()
        .get(header::X_CONTENT_TYPE_OPTIONS)
        .expect("X-Content-Type-Options missing");
    assert_eq!(xcto, "nosniff");
}

#[tokio::test]
async fn swagger_ui_endpoint_exists() {
    let state = common::test_app_state();

    let app = build_router(state, vec![]);
    let request = Request::builder().uri("/docs").body(Body::empty()).unwrap();

    let response = app.oneshot(request).await.unwrap();
    // Swagger UI redirects or serves — either 200 or 3xx is acceptable
    let status = response.status();
    assert!(
        status == StatusCode::OK || status.is_redirection(),
        "Expected 200 or redirect, got {status}"
    );
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let state = common::test_app_state();

    let app = build_router(state, vec![]);
    let request = Request::builder()
        .uri("/nonexistent")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
