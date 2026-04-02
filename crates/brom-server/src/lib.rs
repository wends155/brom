//! Axum REST API and Server components for the brom headless CMS framework.

pub mod error;
pub use error::ServerError;

use axum::Router;
use tower_http::trace::TraceLayer;

/// Creates the API router for a set of registered schemas.
pub fn create_router() -> Router {
    // STUB(Phase 4): Implement dynamic router mapping via SchemaRegistry
    Router::new().layer(TraceLayer::new_for_http())
}
