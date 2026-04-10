use axum::{
    body::Body,
    http::{Response, StatusCode, Uri, header},
    response::IntoResponse,
};
use mime_guess;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../../admin/dist"]
pub struct AdminUi;

/// Handler that serves the embedded Admin SPA assets.
/// If a file is not found, it falls back to serving `index.html` to support client-side routing.
pub async fn spa_fallback(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    // 1. Try serving the exact path
    if let Some(content) = AdminUi::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();
        return Response::builder()
            .header(header::CONTENT_TYPE, mime.as_ref())
            .body(Body::from(content.data))
            .unwrap_or_else(|_| {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            });
    }

    // 2. Fallback to index.html for CSR routing
    if let Some(content) = AdminUi::get("index.html") {
        return Response::builder()
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(content.data))
            .unwrap_or_else(|_| {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            });
    }

    // 3. Absolute fallback
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}
