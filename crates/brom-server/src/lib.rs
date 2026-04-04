//! Axum REST API and Server components for the brom headless CMS framework.

pub mod error;
pub use error::ServerError;

use axum::Router;
use tower_http::trace::TraceLayer;

/// Creates the API router for a set of registered schemas.
pub fn create_router() -> Router {
    use axum::extract::MatchedPath;
    use axum::http::{Request, Response};
    use std::time::Duration;
    use tracing::{Span, info_span};

    // STUB(Phase 4): Implement dynamic router mapping via SchemaRegistry
    Router::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                let path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map_or(request.uri().path(), MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = %request.method(),
                    path,
                )
            })
            .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                tracing::info!(
                    latency = ?latency,
                    status = %response.status().as_u16(),
                    "response_generated"
                );
            }),
    )
}
