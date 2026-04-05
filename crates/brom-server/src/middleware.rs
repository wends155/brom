use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;

/// Returns a CORS layer configured for the admin UI.
///
/// Defaults to permissive for development.
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            // ast-grep-ignore
            HeaderValue::from_static("http://localhost:5173"),
            // ast-grep-ignore
            HeaderValue::from_static("http://localhost:3000"),
            // ast-grep-ignore
            HeaderValue::from_static("http://127.0.0.1:5173"),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::COOKIE])
        .allow_credentials(true)
}

/// Returns a layer that adds common security headers.
pub fn security_headers_layer() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    )
}
